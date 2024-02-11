-- Your SQL goes here
CREATE TABLE session (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc (),
  user_id uuid references "user" (id) on delete cascade not null,
  valid boolean not null default true,
  created_at timestamptz not null default now (),
  updated_at timestamptz
);

-- Applying the 'updated_at' trigger for the 'session' table
SELECT
  trigger_updated_at ('session');
