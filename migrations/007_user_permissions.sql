DO $$ BEGIN CREATE TYPE user_permissions AS ENUM ('USER', 'OWNER', 'ADMINISTRATOR');
EXCEPTION
WHEN duplicate_object THEN null;
END $$;