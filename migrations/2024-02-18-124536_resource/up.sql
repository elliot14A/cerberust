-- Your SQL goes here
create table "resource" (
  id uuid primary key default uuid_generate_v4 (),
  parent_resource_id uuid references "resource" (id) on delete cascade default null,
  name text collate "case_insensitive" not null,
  description text default null,
  created_at timestamptz default now () not null,
  updated_at timestamptz
);

alter table "resource" add constraint "resource_name_parent_resource_id_unique" unique (name, parent_resource_id);
-- applying the 'updated_at' trigger for the 'resource' table
select
  trigger_updated_at ('"resource"');

