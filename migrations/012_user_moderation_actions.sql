DO $$ BEGIN
    CREATE TYPE action_taken AS ENUM ('BANNED_USER', 'KICKED_USER');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS user_moderation_actions (
    action_id       uuid PRIMARY KEY,
    user_id         bigint NOT NULL UNIQUE,
    action_taken    action_taken
);