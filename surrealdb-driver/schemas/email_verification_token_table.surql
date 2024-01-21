DEFINE TABlE email_verification_token SCHEMAFULL;

DEFINE FIELD token ON email_verification_token TYPE string;
DEFINE FIELD user ON email_verification_token TYPE record<user>;
DEFINE FIELD created_at ON email_verification_token TYPE datetime DEFAULT time::now();
