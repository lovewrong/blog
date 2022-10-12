--
-- Table structure for table "users"
--
create table users
(
    -- Having the table name as part of the primary key column makes it nicer to write joins.
    user_id 	  uuid primary key 								 default uuid_generate_v1mc(),
    
    -- By applying our custom collation we can simply mark this column as `unique` and Postgres will enforce.
    -- Lookups over `username` will be case-insensitive by default.
    username 	  text collate "case_insensitive" unique not null,
    
    -- Same for email.
    email    	  text collate "case_insensitive" unique not null,
    
    bio 		  text 									 not null default '',
    
    url   		  text,
    
    -- The Argon2 hashed password string for the user.
    password_hash text                                   not null,
    
    groups		  smallint 								 not null default 1,
    
    disabled	  boolean								 not null default false,
    
    created_at    timestamptz                            not null default now(),

    updated_at    timestamptz
);

-- And applying our `updated_at` trigger is as easy as this.
SELECT trigger_updated_at('users');

-- Create a default sorted index to support search.
-- select * from users where (username collate "ucs_basic") ilike ($1 || '%')
create index on users (username collate "ucs_basic");


--
-- Table structure for table "user_options"
--
create table user_options
(
    user_option_id uuid primary key default uuid_generate_v1mc(),
    
    user_id        uuid not null references users (user_id) on delete cascade,
    
    option_name	   text	not null,
    
    option_value   text
);