-- Your SQL goes here
create table refresh_token (
  id uuid primary key default uuid_generate_v1mc (),
  token text not null,
  session_id uuid references session (id) not null,
  created_at timestamptz not null default now ()
);
