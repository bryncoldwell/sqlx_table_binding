CREATE TABLE public.my_table
(
    id TEXT PRIMARY KEY
)
WITH ( OIDS = FALSE )
TABLESPACE pg_default;

ALTER TABLE public.my_table
    OWNER TO username;
