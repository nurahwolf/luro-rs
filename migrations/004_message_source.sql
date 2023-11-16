DO $$ BEGIN CREATE TYPE message_source AS ENUM (
    'MESSAGE',
    'CUSTOM',
    'CACHED_MESSAGE',
    'MESSAGE_UPDATE',
    'MESSAGE_DELETE',
    'MESSAGE_CREATE',
    'NONE'
);
EXCEPTION
WHEN duplicate_object THEN null;
END $$;