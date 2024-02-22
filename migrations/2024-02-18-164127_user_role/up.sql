-- Your SQL goes here
create table "user_role" (
  id uuid primary key default uuid_generate_v4 (),
  user_id uuid references "user" (id) on delete cascade not null,
  role_id uuid references "role" (id) on delete cascade not null
);

