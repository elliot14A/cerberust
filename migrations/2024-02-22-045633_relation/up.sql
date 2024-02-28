-- Your SQL goes here
create table "relation" (
  user_id uuid references "user" (id) on delete cascade not null,
  object_id uuid not null,
  role_id uuid references "role" (id) on delete cascade not null,
  primary key (user_id, object_id, role_id)
);
