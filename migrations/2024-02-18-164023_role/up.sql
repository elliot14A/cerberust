create table "role" (
  id uuid primary key default uuid_generate_v4 (),
  name text collate "case_insensitive" not null,
  description text default null,
  privileges jsonb default '[]' not null,
  is_default bool default false not null,
  resource_id uuid references resource (id) on delete cascade default null,
  created_at timestamptz default now () not null,
  updated_at timestamptz
);

alter table "role" add constraint "role_name_resource_id_unique" unique (name, resource_id);

select
  trigger_updated_at ('"role"');


insert into "role" (name, description, privileges, is_default) values ('root', 'root role can do anything i.e create, read, update, delete, grant and revoke', '[{"entity": "resource", "privileges": ["*"]}, {"entity": "role", "privileges": ["*"]}]', true);
