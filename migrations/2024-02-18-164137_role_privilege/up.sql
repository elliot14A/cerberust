-- Your SQL goes here
create table role_privilege (
  role_id uuid references "role" (id) on delete cascade not null,
  privilege_id uuid references "privilege" (id) on delete cascade not null,
  resource_id uuid references "resource" (id) on delete cascade not null,
  created_at timestamptz default now (),
  updated_at timestamptz,
  primary key (role_id, privilege_id, resource_id)
);

select
  trigger_updated_at ('"role_privilege"');
