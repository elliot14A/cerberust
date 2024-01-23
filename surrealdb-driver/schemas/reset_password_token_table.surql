DEFINE TABlE reset_password_token SCHEMAFULL;

DEFINE FIELD token ON reset_password_token TYPE string;
DEFINE FIELD user ON reset_password_token TYPE record<user>;
DEFINE FIELD created_at ON reset_password_token TYPE datetime DEFAULT time::now();
