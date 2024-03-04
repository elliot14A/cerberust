# Cerberust 🦀

### About the Project

Cerberust is a reliable Rust authentication server, safeguards your application effortlessly. Using JSON Web Tokens (JWT) for seamless logins, it also offers hierarchical resource management and role-based access controls, ensuring security without the hassle of complex setups.

### Tech Stack
- Rust Axum
- Postgres
- Diesel

### Features
- Fast Authentication 🚀: Swift and efficient user authentication.
- Refresh Tokens 🔐: Enhances security and extends user sessions.
- Refresh Token Rotation 🔄: Regularly updates refresh tokens for heightened security.
- Refresh Token Reuse Detection 🚫: Blocks unauthorized reuse of refresh tokens.
- Database Support 🗃️: Compatible with PostgreSQL 🐘.
- Email Verification 📧: Sends verification emails for account confirmation.
- Password Reset 🔑: Securely resets user passwords.
- ORM Integration 🛠️: Seamless integration with Diesel for database management.
- Hierarchical Resource Management 🌐: Create and manage hierarchical resources.
- Role-Based Access Control (RBAC) 🛡️: Define and manage roles to control access to resources effectively.

### Database structure

![](https://github.com/elliot14A/cerberust/blob/main/assets/database.svg)
![](https://github.com/elliot14A/cerberust/blob/main/assets/cerberust.png)
__token entity__: It is used for storing email-verification-tokens/reset-password-token in the database.

__Note: Account entity not yet fully implemented__


### Getting Started

- __Clone the repo:__

```shell
git clone https://github.com/elliot14A/cerberust
```

- __Run Postgres migrations:__
	 *this step will be removed in the future versions*

```shell
export DATABASE_URL=postgres://postges:postgres@localhost/cerberust
diesel migration run
```

- __Create `cerberust.toml` file and define your roles and resources__

- __Run the application with docker-compose:__

```shell
docker compose up
```

### Configuration
The application can be configured using the `cerberust.toml` file. If this file is not created, the application will use default configurations and will not create any default roles and resources, except the `root` role, which will be created at the migration step itself. Below is an example configuration for `cerberust.toml`:

```toml
[config]
# Port for the application to listen on
port = "8080"

# Database URL
database_url = "postgres://postgres:postgres@postgres/cerberust"

# SMTP host for sending verification emails
smtp_host = "mailhog"

# SMTP port for sending verification emails
smtp_port = "1025"

# Define resources and roles only if you set up a root user
# Pass root user and password as environment variables

[[resource]]
name = "Book Vault"
description = "A place to store your books"

[[resource]]
name = "Into the Wild"
description = "A book about a guy who goes into the wild"
parent = "Book Vault"

[[resource]]
name = "Into Thin Air"
description = "A book about a guy who goes up a mountain"
parent = "Book Vault"

[[resource]]
name = "The Hobbit"
description = "A book about a hobbit"
parent = "Book Vault"

[[role]]
name = "Reader"
privileges = [{ entity = "resource", privilege = ["read"] }]
description = "Can read books"

[[role]]
name = "Writer"
privileges = [{ entity = "resource", privilege = ["read", "create"] }]
description = "Can read and create new books in book vault"
```

To create resources, you need to export ROOT_EMAIL and ROOT_PASSWORD environment variables.

```shell
export ROOT_EMAIL=root@example.com
export ROOT_PASSWORD=rootpassword
```


### Roadmap

- [ ]  Integration Tests  
- [ ]  Add API Endpoint documentation
- [ ]  Social Auth
    - [ ]  Google
    - [ ]  Github
    - [ ]  Facebook
    - [ ]  Twitter
- [x] RBAC Feature
- [ ] Proper Error Handling and Logging
- [x] Docker Image
- [ ] User management

### Contributing

Contributions are always welcome!

See `CONTRIBUTING.md` for ways to get started.

### Code of Conduct

Please read the `Code_of_Conduct.md`

### License

This project is licensed under WTFPL. See `LICENSE` for more information.

### Contact
Akshith Katkuri - akshithmadhur0072@gmail.com

project link: https://github.com/elliot14a/cerberust
