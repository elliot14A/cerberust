-- Your SQL goes here
create table "privilege" (
  id uuid primary key default uuid_generate_v4 (),
  name text collate "case_insensitive" not null,
  description text default null,
  created_at timestamptz default now () not null,
  updated_at timestamptz
);

select
  trigger_updated_at ('"privilege"');

-- default privileges create, read, update, delete

insert into "privilege" (name, description) values ('create', 'create resource');
insert into "privilege" (name, description) values ('read', 'read resource');
insert into "privilege" (name, description) values ('update', 'update resource');
insert into "privilege" (name, description) values ('delete', 'delete resource');
