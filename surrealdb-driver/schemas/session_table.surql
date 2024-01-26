DEFINE TABLE session SCHEMAFULL;

DEFINE FIELD user ON session TYPE record<user>;
DEFINE FIELD valid ON session TYPE bool DEFAULT true;
DEFINE FIELD created_at ON session TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON session TYPE datetime DEFAULT time::now() VALUE time::now();
