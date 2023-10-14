CREATE TABLE IF NOT EXISTS user_characters (
    character_id        uuid PRIMARY KEY,
    nsfw_description    text,
    nsfw_icons          text[],
    nsfw_summary        text,
    prefixes            text[],
    sfw_description     text NOT NULL,
    sfw_icons           text[] NOT NULL,
    sfw_summary         text NOT NULL
);