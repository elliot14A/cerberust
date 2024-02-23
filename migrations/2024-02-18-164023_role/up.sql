create table "role" (
  id uuid primary key default uuid_generate_v4 (),
  name text collate "case_insensitive" unique not null,
  description text default null,
  privileges jsonb default '[]' not null,
  created_at timestamptz default now () not null,
  updated_at timestamptz
);

select
  trigger_updated_at ('"role"');

insert into "role" (name, description, privileges) values ('root', 'root role can do anything i.e create, read, update, delete, grant and revoke', '[{"entity": "resource", "privileges": ["*"]}, {"entity": "role", "privileges": ["*"]}]');
