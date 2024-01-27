# Cerberust  🐾

### About the Project

Cerberust is a formidable authentication server written in Rust, designed to be the vigilant guardian of your application's security. Built with a focus on speed and advanced security measures, Cerberust ensures a seamless and protected authentication experience. Leveraging JSON Web Tokens (JWT) for authentication, this server provides a reliable and efficient solution for user authentication. 

### Tech Stack
- Rust Axum
- SurrealDB
- Postgres

### Features
- __Fast Authentication 🚀__: Built with a focus on speed, Cerberust provides rapid and efficient user authentication.
- __Refresh Tokens 🔐__: Implements refresh tokens for enhanced security and extended user sessions.
- __Refresh Token Rotation 🔄__: Enhances security by rotating refresh tokens, minimizing the risk of unauthorized access.
- __Refresh Token Reuse Detection 🚫__: Detects and prevents the reuse of refresh tokens, adding an extra layer of protection.
- __Database Support 🗃️__:
    a. Currently supports SurrealDB 📊: A fast and efficient graph database.
    b. PostgreSQL 🐘 support is in active development. 
- __Email Verification 📧__: Sends verification emails to new users for email confirmation.
- __Password Reset 🔑__: Allows users to reset their passwords through a secure process.

### Endpoints
- GET /api/health - Check Health
- POST /api/register - register 
- POST /api/login - login
- POST /api/logout - logout
- POST /api/refresh - refresh access token
- POST /api/forgot_password - send reset password email
- POST /api/reset_password/:token - reset password
- POST /api/resend - resend verification email
- POST, GET /api/verify/:token - verify email
- POST, GET /api/whoami - get user info from access token


### Database structure

![](https://github.com/elliot14A/cerberust/blob/main/assets/database.svg)

__token entity__: It is used for storing email-verification-tokens/reset-password-token in the database.

__Note: Account entity not yet fully implemented__


### Getting Started

- __Clone the repo:__

```shell
git clone https://github.com/elliot14A/cerberust
```

- __Run SurrealDB migrations:__
	 *this step will be removed in the future versions*

```shell
cd cerberust/surrealdb-driver
cargo install surrealdb-migrations
surrealdb-migrations apply
```

- __Run the application with docker-compose:__

```shell
cd ..
docker compose up
```



### Roadmap

- [ ]  Integration Tests  
- [ ]  Add API Endpoint documentation
- [ ]  Social Auth
    - [ ]  Google
    - [ ]  Github
    - [ ]  Facebook
    - [ ]  Twitter
- [ ] RBAC Feature
- [ ] PostgresDB Support 
- [ ] Surrealdb Support
	- [x] Http Client
	- [ ] Ws Client
	- [ ] Embedded
- [ ] Proper Error Handling and Logging
- [ ] Docker Image


### Contributing

Contributions are always welcome!

See `CONTRIBUTING.md` for ways to get started.

### Code of Conduct

Please read the `Code_of_Conduct.md`

### License

This project is licensed under WTFPL. See `LICENSE` for more information.

### Contact
Akshith Katkuri - akshithmadhur0072@gmail.com

Project Link: https://github.com/elliot14A/cerberust
