-- Your SQL goes here
create table "role" (
  id uuid primary key default uuid_generate_v4 (),
  name text collate "case_insensitive" not null,
  description text default null,
  created_at timestamptz default now () not null,
  updated_at timestamptz
);

select
  trigger_updated_at ('"role"');

insert into "role" (name, description) values ('owner', 'owner of the resource');
