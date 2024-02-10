use std::io::Write;

use crate::schema::token;
use chrono::{DateTime, Utc};
use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    prelude::*,
    serialize::{self, IsNull, Output, ToSql},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = crate::schema::sql_types::TokenTypeEnum)]
pub enum TokenType {
    VerifyEmail,
    ResetPassword,
}

impl ToSql<crate::schema::sql_types::TokenTypeEnum, Pg> for TokenType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            TokenType::VerifyEmail => out.write_all(b"email_verification_token")?,
            TokenType::ResetPassword => out.write_all(b"reset_password_token")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::TokenTypeEnum, Pg> for TokenType {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"email_verification_token" => Ok(TokenType::VerifyEmail),
            b"reset_password_token" => Ok(TokenType::ResetPassword),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = token)]
pub struct NewToken<'a> {
    pub user_id: Uuid,
    pub token_text: &'a str,
    pub token_type: TokenType,
}

#[derive(Debug, Queryable, Serialize, Selectable)]
#[diesel(table_name = token)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Token {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_text: String,
    pub token_type: TokenType,
    pub created_at: DateTime<Utc>,
}
