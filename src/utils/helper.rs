use crate::models::{
    role::{Privilege, PrivilegeVec},
    CREATE, DELETE, GRANT, READ, RESOURCE, REVOKE, ROLE, UPDATE,
};

pub fn filter_privileges(privileges: Vec<Privilege>) -> PrivilegeVec {
    let mut privilege_vec = vec![
        Privilege {
            entity: RESOURCE.to_string(),
            privileges: vec![],
        },
        Privilege {
            entity: ROLE.to_string(),
            privileges: vec![],
        },
    ];

    const ALLOWED_ROLE_PRIVILEGES: [&str; 5] = [REVOKE, GRANT, UPDATE, DELETE, CREATE];
    const ALLOWED_RESOURCE_PRIVILEGES: [&str; 4] = [CREATE, UPDATE, DELETE, READ];

    for privilege in privileges {
        match &privilege.entity[..] {
            ROLE => {
                for priv_name in &privilege.privileges {
                    if ALLOWED_ROLE_PRIVILEGES.contains(&priv_name.as_str()) {
                        privilege_vec[1].privileges.push(priv_name.clone());
                    }
                }
            }
            RESOURCE => {
                for priv_name in &privilege.privileges {
                    if ALLOWED_RESOURCE_PRIVILEGES.contains(&priv_name.as_str()) {
                        privilege_vec[0].privileges.push(priv_name.clone());
                    }
                }
            }
            _ => {}
        }
    }

    PrivilegeVec(privilege_vec)
}
