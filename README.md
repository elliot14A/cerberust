# Cerberust 🦀

### About the Project

Cerberust is a formidable authentication server written in Rust, designed to be the vigilant guardian of your application's security. Built with a focus on speed and advanced security measures, Cerberust ensures a seamless and protected authentication experience. Leveraging JSON Web Tokens (JWT) for authentication, this server provides a reliable and efficient solution for user authentication. 

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

### Endpoints
- GET /api/health - Check Health
- Auth routes `/api/auth`
    - POST /register - register 
    - POST /login - login
    - POST, GET /verify/:token - verify email
    - POST /api/resend - resend verification email
- Session routes `/api/session`
    - POST /refresh - refresh access token
    - POST /logout - logout
- Password routes `/api/password`
    - POST /forgot - send reset password email
    - POST /reset/:token - reset password
- User routes `/api/user`
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

- __Run Postgres migrations:__
	 *this step will be removed in the future versions*

```shell
diesel migration run
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
