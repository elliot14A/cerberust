DEFINE TABlE token SCHEMAFULL;

DEFINE FIELD token ON token TYPE string;
DEFINE FIELD user ON token TYPE record<user>;
DEFINE FIELD created_at ON token TYPE datetime DEFAULT time::now();
DEFINE FIELD token_type ON token TYPE string;
