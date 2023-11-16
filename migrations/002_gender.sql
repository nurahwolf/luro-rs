DO $$ BEGIN CREATE TYPE gender AS ENUM (
    'MALE',
    'FEMALE',
    'TRANS_FEMALE',
    'TRANS_MALE',
    'ITS_COMPLICATED'
);
EXCEPTION
WHEN duplicate_object THEN null;
END $$;
