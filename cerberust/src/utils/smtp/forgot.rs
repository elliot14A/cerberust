use lettre::{
    message::{header::ContentType, MultiPart, SinglePart},
    Transport,
};

use crate::error::{ApiErrResp, Result};

use super::SmtpService;

pub async fn send_forgot_password_mail(
    smtp: &SmtpService,
    to: String,
    token: String,
) -> Result<()> {
    let default_http_port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let default_addr = std::env::var("HOST").unwrap_or_else(|_| "http://0.0.0.0".to_string());
    let default_host = format!("{}:{}", default_addr, default_http_port);
    let forgot_password_link = format!("{}/api/forgot_password/{}", default_host, token);

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
    let message = lettre::Message::builder()
        .from(format!("{} <{}>", "Cerberust", smtp.email).parse().unwrap())
        .to(to.parse().unwrap())
        .subject("Forgot Password")
        .multipart(
            MultiPart::alternative().singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(html.to_string()),
            ),
        )
        .unwrap();
    smtp.transport
        .send(&message)
        .map_err(|e| ApiErrResp::internal_server_error(e.to_string()))?;
    Ok(())
}
