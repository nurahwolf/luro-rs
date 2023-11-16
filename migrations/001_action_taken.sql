DO $$ BEGIN CREATE TYPE action_taken AS ENUM ('BANNED_USER', 'KICKED_USER');
EXCEPTION
WHEN duplicate_object THEN null;
END $$;