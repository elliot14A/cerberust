use anyhow::anyhow;
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
    },
    models::{
        relation::NewRelation,
        resource::NewResource,
        role::{NewRole, Privilege},
        user::NewUser,
        ROOT_ROLE,
    },
    utils::{hash::hash_password, helper::filter_privileges},
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
    pub privileges: Vec<Privilege>,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Config {
    #[serde(rename = "config")]
    pub server_config: ServerConfig,
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

        Ok(Some(root_user.id))
    }

    pub async fn create_resources(
        &self,
        conn: &mut AsyncPgConnection,
        root_user_id: Uuid,
    ) -> anyhow::Result<()> {
        if self.resources.is_empty() {
            info!("📦 No resources to create.");
            return Ok(());
        }

        let role_id = get_role_id_by_name(conn, ROOT_ROLE).await?.ok_or_else(|| {
            error!("🚫 Root role not found.");
            anyhow!("Root role not found")
        })?;

        for resource in &self.resources {
            info!("📦 Creating resource: {}", resource.name);

            let resource_id = if let Some(parent_name) = &resource.parent {
                let parent_resource_id =
                    get_resource_id_by_name(conn, parent_name.as_str()).await?;
                match parent_resource_id {
                    Some(parent_id) => {
                        Self::_create_resource(conn, resource, Some(parent_id)).await?
                    }
                    None => {
                        error!("📦 Parent resource '{}' not found.", parent_name);
                        continue;
                    }
                }
            } else {
                Self::_create_resource(conn, resource, None).await?
            };

            if let Some(id) = resource_id {
                info!("📦 Resource {} created successfully!", resource.name);
                let new_relation = NewRelation {
                    user_id: root_user_id,
                    role_id,
                    resource_id: id,
                };

                create_relation(conn, new_relation).await?;

                info!(
                    "🔗 Relation between {} and root user created successfully.",
                    resource.name
                );
            }
        }

        Ok(())
    }

    async fn _create_resource(
        conn: &mut AsyncPgConnection,
        resource: &Resource,
        parent_id: Option<Uuid>,
    ) -> anyhow::Result<Option<Uuid>> {
        let new_resource = NewResource {
            name: resource.name.clone(),
            description: resource.description.clone(),
            parent_resource_id: parent_id,
        };

        let resource = create_resource(conn, new_resource.clone()).await;
        match resource {
            Ok(resource) => Ok(Some(resource.id)),
            Err(e) => {
                if e.error == "CONFLICT" {
                    info!(
                        "📦 Resource {} already exists, skipping!",
                        new_resource.name
                    );
                    Ok(None) // Return parent_id as resource_id
                } else {
                    Err(e.into())
                }
            }
        }
    }

    pub async fn create_roles(&self, conn: &mut AsyncPgConnection) -> anyhow::Result<()> {
        if self.roles.is_empty() {
            info!("🧙 No roles to create.");
            return Ok(());
        }
        for role in &self.roles {
            info!("🧙 Creating role {}", role.name);
            let privileges = filter_privileges(role.privileges.clone());
            let new_role = NewRole {
                name: role.name.clone(),
                description: role.description.clone(),
                privileges,
                resource_id: None,
                is_default: true,
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
}
