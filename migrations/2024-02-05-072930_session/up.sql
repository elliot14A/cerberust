-- Your SQL goes here
create table session (
  id uuid primary key default uuid_generate_v4 (),
  user_id uuid references "user" (id) on delete cascade not null,
  valid boolean not null default true,
  created_at timestamptz not null default now (),
  updated_at timestamptz
);

-- applying the 'updated_at' trigger for the 'session' table
select
  trigger_updated_at ('session');
