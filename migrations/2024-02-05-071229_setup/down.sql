-- This file should undo anything in `up.sql`
-- drop the trigger and function
drop trigger if exists set_updated_at on your_table_name;

drop function if exists set_updated_at ();

-- drop the collation
drop collation if exists case_insensitive;
