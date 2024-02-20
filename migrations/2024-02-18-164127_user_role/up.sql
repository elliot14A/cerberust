-- Your SQL goes here
create table "user_role" (
  user_id uuid references "user" (id) on delete cascade not null,
  role_id uuid references "role" (id) on delete cascade not null,
  created_at timestamptz default now () not null,
  updated_at timestamptz,
  primary key (user_id, role_id)
);

select
  trigger_updated_at ('"user_role"');
