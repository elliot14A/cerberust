DEFINE TABlE refresh_token SCHEMAFULL;

DEFINE FIELD token ON refresh_token TYPE string;
DEFINE FIELD session ON refresh_token TYPE record<session>;
DEFINE FIELD created_at ON refresh_token TYPE datetime DEFAULT time::now();
