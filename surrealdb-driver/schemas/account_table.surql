DEFINE TABLE account SCHEMAFULL;

DEFINE FIELD user ON account TYPE record<user>;
DEFINE FIELD account_type ON account TYPE string;
DEFINE FIELD provider ON account TYPE string;
DEFINE FIELD provider_account_id ON account TYPE option<string>;
DEFINE FIELD refresh_token ON account TYPE option<string>;
DEFINE FIELD access_token ON account TYPE option<string>;
DEFINE FIELD expires_at ON account TYPE option<datetime>;
DEFINE FIELD scope ON account TYPE option<string>;
DEFINE FIELD id_token ON account TYPE option<string>;
DEFINE FIELD session_state ON account TYPE option<string>;
DEFINE FIELD created_at ON account TYPE datetime default time::now();
DEFINE FIELD updated_at ON account TYPE datetime DEFAULT time::now() VALUE time::now();
