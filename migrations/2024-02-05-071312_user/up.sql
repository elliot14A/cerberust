-- Your SQL goes here
create table "user" (
  id uuid primary key default uuid_generate_v4 (),
  username text collate "case_insensitive" not null,
  email text collate "case_insensitive" unique not null,
  password text not null,
  email_verified boolean not null default false,
  created_at timestamptz not null default now (),
  updated_at timestamptz
);

-- And applying our `updated_at` trigger is as easy as this.
select
  trigger_updated_at ('"user"');
