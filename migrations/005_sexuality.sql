DO $$ BEGIN CREATE TYPE sexuality AS ENUM (
    'STRAIGHT',
    'BISEXUAL',
    'PANSEXUAL',
    'LESBIAN',
    'GAY'
);
EXCEPTION
WHEN duplicate_object THEN null;
END $$;