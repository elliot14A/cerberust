-- Your SQL goes here
-- Create an ENUM for token_type
create type token_type_enum as enum (
  'email_verification_token',
  'reset_password_token'
);

-- create the 'token' table
create table "token" (
  id uuid primary key default uuid_generate_v1mc (),
  user_id uuid references "user" (id) not null,
  token text not null,
  token_type token_type_enum not null
);
