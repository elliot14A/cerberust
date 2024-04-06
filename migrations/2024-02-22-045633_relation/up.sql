-- Your SQL goes here
create table "relation" (
  user_id uuid references "user" (id) on delete cascade not null,
  resource_id uuid references "resource" (id) on delete cascade not null,
  role_id uuid references "role" (id) on delete cascade not null,
  primary key (user_id, resource_id)
);
