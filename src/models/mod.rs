pub mod refresh_token;
pub mod relation;
pub mod resource;
pub mod role;
pub mod session;
pub mod token;
pub mod user;
pub mod user_role;

// default role
pub const ROOT_ROLE: &str = "root";

// default privileges
pub const CREATE: &str = "create";
pub const READ: &str = "read";
pub const UPDATE: &str = "update";
pub const DELETE: &str = "delete";
pub const ROOT: &str = "*";
pub const GRANT: &str = "grant";
pub const REVOKE: &str = "revoke";

// entities
pub const RESOURCE: &str = "resource";
pub const ROLE: &str = "role";
