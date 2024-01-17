DEFINE TABLE user SCHEMAFULL;

DEFINE FIELD name ON user TYPE string;
DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value) VALUE string::lowercase($value);
DEFINE FIELD password ON user TYPE string;
DEFINE FIELD email_verified ON user TYPE bool DEFAULT false;
DEFINE FIELD created_at ON user TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON user TYPE datetime DEFAULT time::now();
DEFINE FIELD accounts ON user TYPE array<record<account>>;
DEFINE FIELD accounts.* ON user TYPE record<account>;
DEFINE FIELD avatar ON user TYPE record<avatar>;

DEFINE INDEX unique_username ON user COLUMNS username UNIQUE;
DEFINE INDEX unique_email ON user COLUMNS email UNIQUE;

DEFINE EVENT updated_at ON TABLE user WHEN $event = "UPDATE" THEN (
  update user SET updated_at = time::now() WHERE id = $value.id
);
