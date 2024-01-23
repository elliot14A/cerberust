use lettre::{
    message::{header::ContentType, MultiPart, SinglePart},
    transport::smtp::{authentication::Credentials, client::Tls},
    SmtpTransport, Transport,
};
use tracing::{error, info};

use crate::error::ApiErrResp;

fn smtp_service() -> SmtpTransport {
    let smtp_host = std::env::var("SMTP_HOST").unwrap_or("mailhog".to_string());
    let smtp_user = std::env::var("SMTP_USER").unwrap_or("".to_string());
    let smtp_pass = std::env::var("SMTP_PASS").unwrap_or("".to_string());
    let smtp_port = std::env::var("SMTP_PORT").unwrap_or("1025".to_string());

    let credentials = Credentials::new(smtp_user, smtp_pass);

    let smtp = SmtpTransport::starttls_relay(&smtp_host)
        .expect("Failed to create transport")
        .tls(Tls::None)
        .port(smtp_port.parse::<u16>().unwrap())
        .credentials(credentials)
        .build();
    let s = smtp.test_connection();
    if s.is_err() {
        println!("error : {:?}", s.err());
        panic!("Failed to connect to SMTP server");
    }
    smtp
}

#[derive(Debug, Clone)]
pub struct SmtpService {
    transport: SmtpTransport,
    email: String,
}

impl SmtpService {
    pub fn new() -> Self {
        Self {
            transport: smtp_service(),
            email: std::env::var("SMTP_EMAIL").unwrap_or("cerberust@example.com".to_string()),
        }
    }

    fn get_host(&self) -> String {
        let default_http_port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
        let default_addr = std::env::var("HOST").unwrap_or_else(|_| "http://0.0.0.0".to_string());
        let default_host = format!("{}:{}", default_addr, default_http_port);
        default_host
    }

    pub fn send_password_reset_email(&self, to: String, token: String) -> Result<(), ApiErrResp> {
        let default_host = self.get_host();
        let forgot_password_link = format!("{}/api/reset_password/{}", default_host, token);
        let html = format!(
            r#"<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Forgot Password</title>
        </head>
        <body>
            <div style="display: flex; flex-direction: column; align-items: center;">
                <h2 style="font-family: Arial, Helvetica, sans-serif;">Email Verification</h2>
                <p style="font-family: Arial, Helvetica, sans-serif;">
                    Click the link below to reset your password:
                </p>
                <a href="{}" style="font-family: Arial, Helvetica, sans-serif; padding: 10px; background-color: #4CAF50; color: white; text-decoration: none; text-align: center; display: inline-block; border-radius: 4px;">
                 Reset
                </a>
            </div>
        </body>
        </html>"#,
            forgot_password_link
        );

        self.send_email_with_retries(html.to_owned(), "Reset password request".to_owned(), to, 0)
    }

    pub fn send_verification_email(&self, to: String, token: String) -> Result<(), ApiErrResp> {
        let default_host = self.get_host();
        let verification_link = format!("{}/api/verify/{}", default_host, token);
        let html = format!(
            r#"<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Email Verification</title>
        </head>
        <body>
            <div style="display: flex; flex-direction: column; align-items: center;">
                <h2 style="font-family: Arial, Helvetica, sans-serif;">Email Verification</h2>
                <p style="font-family: Arial, Helvetica, sans-serif;">
                    Click the link below to verify your email address:
                </p>
                <a href="{}" style="font-family: Arial, Helvetica, sans-serif; padding: 10px; background-color: #4CAF50; color: white; text-decoration: none; text-align: center; display: inline-block; border-radius: 4px;">
                    Verify Email
                </a>
            </div>
        </body>
        </html>"#,
            verification_link
        );

        self.send_email_with_retries(html.to_owned(), "Verification Email".to_owned(), to, 0)
    }

    fn send_email(&self, html: String, subject: String, to: String) -> Result<(), ApiErrResp> {
        let message = lettre::Message::builder()
            .from(format!("{} <{}>", "Cerberust", self.email).parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .multipart(
                MultiPart::alternative().singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_HTML)
                        .body(html),
                ),
            )
            .unwrap();
        self.transport
            .send(&message)
            .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;
        Ok(())
    }

    fn send_email_with_retries(
        &self,
        html: String,
        subject: String,
        to: String,
        retries: i8,
    ) -> Result<(), ApiErrResp> {
        if retries > 5 {
            return Err(ApiErrResp::internal_server_error(
                "Unable to send verification email".to_string(),
            ));
        }
        let result = self.send_email(html.clone(), subject.clone(), to.clone());
        if result.is_err() {
            error!("error : {:?}", result.err());
            info!(
                "Retrying to send verification email for the {} time",
                retries
            );
            self.send_email_with_retries(html, subject, to, retries + 1)?;
        }
        info!("Successfully sent verification email");
        Ok(())
    }
}
