CREATE table usertable(
   username text,
   password text,
   key text,
   userid text NOT NULL,
   CONSTRAINT usertable_pkey PRIMARY KEY (userid),
   CONSTRAINT usertable_username UNIQUE (username)
);

CREATE INDEX IF NOT EXISTS "usertable_userId_idx"
    ON usertable USING btree
    (userid COLLATE pg_catalog."default" ASC NULLS LAST)
    WITH (deduplicate_items=True)
    TABLESPACE pg_default;



Create table session(
   userid  text NOT NULL,
   time bigint NOT NULL,
   token text NOT NULL,
   CONSTRAINT session_pkey PRIMARY KEY (userid),
   CONSTRAINT session_token UNIQUE  (token)
)
