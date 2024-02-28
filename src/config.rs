use std::env;

use diesel_async::AsyncPgConnection;
use serde::Deserialize;
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    actions::{
        relation::create::create_relation,
        resource::{create::create_resource, details::get_resource_id_by_name},
        role::{create::create_role, details::get_role_id_by_name},
        user::{create::create_user, details::get_user_by_email, update::update_email_verified},
        user_role::create::create_user_role,
    },
    models::{
        relation::NewRelation,
        role::{NewRole, Privilege, PrivilegeVec},
        user::NewUser,
        user_role::NewUserRole,
        CREATE, DELETE, GRANT, READ, RESOURCE, REVOKE, ROLE, ROOT_ROLE, UPDATE,
    },
    utils::hash::hash_password,
};

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: String,
    pub database_url: String,
    pub smtp_host: String,
    pub smtp_port: String,
    pub keys_path: Option<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            port: "8080".to_string(),
            database_url: "postgres://postgres:postgres@postgres/cerberust".to_string(),
            smtp_host: "mailhog".to_string(),
            smtp_port: "1025".to_string(),
            keys_path: None,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Resource {
    pub name: String,
    pub description: Option<String>,
    pub parent: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Role {
    pub name: String,
    pub description: Option<String>,
    pub privileges: Vec<String>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Config {
    pub config: ServerConfig,
    #[serde(rename = "resource")]
    pub resources: Vec<Resource>,
    #[serde(rename = "role")]
    pub roles: Vec<Role>,
}

impl Config {
    pub fn load() -> anyhow::Result<(Self, bool)> {
        // parse cerberust.toml file
        let default: bool;
        let config: Config = match std::fs::read_to_string("cerberust.toml") {
            Ok(content) => {
                default = false;
                toml::from_str(&content)?
            }
            Err(_) => {
                default = true;
                Config::default()
            }
        };
        Ok((config, default))
    }

    pub async fn create_root_user(
        &self,
        conn: &mut AsyncPgConnection,
    ) -> anyhow::Result<Option<Uuid>> {
        let email = match env::var("ROOT_EMAIL") {
            Ok(val) => val,
            Err(_) => {
                info!("🔑 ROOT_EMAIL not set");
                return Ok(None);
            }
        };

        let password = match env::var("ROOT_PASSWORD") {
            Ok(val) => val,
            Err(_) => {
                info!("🔑 ROOT_PASSWORD not set");
                return Ok(None);
            }
        };

        let hash = hash_password(password).await?;
        // create root user with email_verified = true
        let new_user = NewUser {
            username: "root".to_string(),
            email: email.clone(),
            password: hash,
        };
        let root_user = create_user(conn, new_user).await;
        let root_user = match root_user {
            Ok(user) => user,
            Err(e) => {
                if e.error == "CONFLICT" {
                    info!("🔑 Root user already exists.");

                    return Ok(Some(get_user_by_email(conn, email).await?.unwrap().id));
                }
                return Err(e.into());
            }
        };

        // update the user email_verified to true
        update_email_verified(conn, root_user.id).await?;

        let role_id = get_role_id_by_name(conn, ROOT_ROLE).await?;

        let role_id = role_id.unwrap();

        let new_user_role = NewUserRole {
            user_id: root_user.id,
            role_id: role_id.clone(),
        };

        create_user_role(conn, new_user_role).await?;
        info!("🔑 Root user created");

        Ok(Some(root_user.id))
    }

    pub async fn create_resources(
        &self,
        root_user_id: Uuid,
        conn: &mut AsyncPgConnection,
    ) -> anyhow::Result<()> {
        if self.resources.is_empty() {
            info!("📦 No resources to create.");
            return Ok(());
        }

        let role_id = get_role_id_by_name(conn, ROOT_ROLE).await?;
        let role_id = role_id.unwrap();

        for resource in &self.resources {
            info!("📦 Creating resource: {}", resource.name);
            let resource_id: Uuid;
            if resource.parent.is_some() {
                let parent_resource_id =
                    get_resource_id_by_name(conn, &resource.parent.clone().unwrap()).await?;
                if parent_resource_id.is_none() {
                    error!("📦 Parent resource not found.");
                    continue;
                }
                let new_resource = crate::models::resource::NewResource {
                    name: resource.name.clone(),
                    description: resource.description.clone(),
                    parent_resource_id,
                };
                let resource = create_resource(conn, new_resource.clone()).await;
                let resource = match resource {
                    Ok(resource) => resource,
                    Err(e) => {
                        if e.error == "CONFLICT" {
                            info!(
                                "📦 Resource {} already exists, skipping!.",
                                new_resource.name
                            );
                            continue;
                        }
                        return Err(e.into());
                    }
                };
                resource_id = resource.id;

                info!("📦 Sub-resource created successfully.");
                // create sub-resource
            } else {
                // create resource
                let new_resource = crate::models::resource::NewResource {
                    name: resource.name.clone(),
                    description: resource.description.clone(),
                    parent_resource_id: None,
                };

                let resource = create_resource(conn, new_resource.clone()).await;
                let resource = match resource {
                    Ok(resource) => resource,
                    Err(e) => {
                        if e.error == "CONFLICT" {
                            info!(
                                "📦 Resource {} already exists, skipping!.",
                                new_resource.name
                            );
                            continue;
                        }
                        return Err(e.into());
                    }
                };
                resource_id = resource.id;

                info!("📦 Resource created successfully.");
            }

            // create relatio between root user and resource
            let new_relation = NewRelation {
                user_id: root_user_id,
                role_id,
                object_id: resource_id,
            };

            create_relation(conn, new_relation).await?;

            info!(
                "🔗 Relation between {} and root user created successfully.",
                resource.name
            );
        }

        Ok(())
    }

    pub async fn create_roles(&self, conn: &mut AsyncPgConnection) -> anyhow::Result<()> {
        if self.roles.is_empty() {
            info!("🧙 No roles to create.");
            return Ok(());
        }
        for role in &self.roles {
            info!("🧙 Creating role {}", role.name);
            let privileges = Self::create_privileges_vec(role.privileges.clone());
            let new_role = NewRole {
                name: role.name.clone(),
                description: role.description.clone(),
                privileges,
            };
            let res = create_role(conn, new_role).await;
            if let Err(e) = res {
                if e.error == "CONFLICT" {
                    info!("🧙 Role {} already exists. skipping!", role.name);
                    continue;
                }
                return Err(e.into());
            }
            info!("🧙 Role {} created successfully.", role.name);
        }
        Ok(())
    }

    fn create_privileges_vec(privileges: Vec<String>) -> PrivilegeVec {
        let mut privilege_vec = Vec::new();
        privilege_vec.push(Privilege {
            entity: RESOURCE.to_string(),
            privileges: vec![],
        });
        privilege_vec.push(Privilege {
            entity: ROLE.to_string(),
            privileges: vec![],
        });

        privileges
            .iter()
            .for_each(|privilege| match privilege.as_str() {
                CREATE | READ | UPDATE | DELETE => {
                    privilege_vec[0].privileges.push(privilege.to_string());
                }
                GRANT | REVOKE => {
                    privilege_vec[1].privileges.push(privilege.to_string());
                }
                _ => {}
            });
        PrivilegeVec(privilege_vec)
    }
}
