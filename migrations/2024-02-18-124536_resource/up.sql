-- Your SQL goes here
create table "resource" (
  id uuid primary key default uuid_generate_v4 (),
  parent_resource_id uuid references "resource" (id) on delete cascade default null,
  created_by_id uuid references "user" (id) on delete cascade not null, 
  name text collate "case_insensitive" unique not null,
  description text default null,
  created_at timestamptz default now () not null,
  updated_at timestamptz
);

-- applying the 'updated_at' trigger for the 'resource' table
select
  trigger_updated_at ('"resource"')
