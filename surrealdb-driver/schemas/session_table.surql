DEFINE TABLE session SCHEMAFULL;

DEFINE FIELD user ON session TYPE string;
DEFINE FIELD refresh_tokens ON session TYPE array<record<refresh_token>>;
DEFINE FILED refresh_tokens.* ON session TYPE recorde<refresh_token>;
DEFINE FIELD valid ON session TYPE bool;
DEFINE FIELD created_at ON session TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON session TYPE datetime DEFAULT time::now() VALUE time::now();
