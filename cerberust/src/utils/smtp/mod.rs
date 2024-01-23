mod forgot;
mod verify;

use lettre::{
    transport::smtp::{authentication::Credentials, client::Tls},
    SmtpTransport,
};

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

    pub fn send_verification_email(&self, to: String, token: String) -> Result<(), ApiErrResp> {
        verify::verification_email(self, to, token)
    }
}
