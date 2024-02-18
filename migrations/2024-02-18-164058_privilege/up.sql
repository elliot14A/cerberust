-- Your SQL goes here
create table "privilege" (
  id uuid primary key default uuid_generate_v4 (),
  name text collate "case_insensitive" not null,
  description text default null,
  created_at timestamptz default now (),
  updated_at timestamptz
);

select
  trigger_updated_at ('"privilege"');
