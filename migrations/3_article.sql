--
-- Table structure for table "articles"
--
create table articles
(
    article_id    uuid primary key 	     default uuid_generate_v1mc(),
    
    user_id       uuid 		    not null references users (user_id) on delete cascade,
    
    slug          text unique   not null,

    title         text          not null,

    description   text          not null,

    content       text          not null,

    html          text          not null,

    views         int           not null default 0,
    
    comment_count int		    not null default 0,
    
    allow_comment boolean       not null default true,

    created_at    TIMESTAMPTZ   not null default now(),

    updated_at    TIMESTAMPTZ   not null default now()
);

select trigger_updated_at('articles');

--
-- Table structure for table "comments"
--
create table comments
(
    comment_id uuid primary key 	     default uuid_generate_v1mc(),
    
    article_id uuid        not null references articles (article_id) on delete cascade,
    
    user_id    uuid        not null references users (user_id) on delete cascade,
    
    content    text		   not null,
    
    created_at    TIMESTAMPTZ   not null default now(),

    updated_at    TIMESTAMPTZ   not null default now()
);

select trigger_updated_at('comments');

create index on comments (article_id, created_at);

--
-- Table structure for table "replys"
--
create table replys
(
    reply_id 		uuid primary key 	     default uuid_generate_v1mc(),
    
    comment_id    	uuid         not null references comments (comment_id) on delete cascade,
    
    user_id			uuid         not null references users (user_id) on delete cascade,
    
    content    		text		 not null,
    
    created_at    	TIMESTAMPTZ  not null 	 default now(),

    updated_at   	TIMESTAMPTZ  not null    default now()
);

select trigger_updated_at('replys');

create index on comments (comment_id, created_at);