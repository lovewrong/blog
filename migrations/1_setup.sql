-- Use `uuid_ generate_ V1mc()` instead of `gen_ random_ uuid()`.
create extension if not exists "uuid-ossp";

-- An update trigger for 'updated_at'.
-- usage: select trigger_updated_at('<table name>');
create or replace function set_updated_at()
    returns trigger as
$$
begin
    NEW.updated_at = now();
    return NEW;
end;
$$ language plpgsql;

create or replace function trigger_updated_at(tablename regclass)
    returns void as
$$
begin
    execute format('CREATE TRIGGER set_updated_at
        BEFORE UPDATE
        ON %s
        FOR EACH ROW
        WHEN (OLD is distinct from NEW)
    EXECUTE FUNCTION set_updated_at();', tablename);
end;
$$ language plpgsql;

-- A text collation that sorts text case-insensitively, useful for `UNIQUE` indexes.
create collation case_insensitive (provider = icu, locale = 'und-u-ks-level2', deterministic = false);