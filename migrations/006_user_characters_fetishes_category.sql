DO $$ BEGIN CREATE TYPE user_characters_fetishes_category AS ENUM (
    'FAV',
    'LOVE',
    'LIKE',
    'NEUTRAL',
    'DISLIKE',
    'HATE',
    'LIMIT'
);
EXCEPTION
WHEN duplicate_object THEN null;
END $$;