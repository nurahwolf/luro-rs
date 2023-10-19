DROP TABLE user_characters;
CREATE TABLE IF NOT EXISTS user_characters (
    user_id         bigint references users(user_id) ON UPDATE CASCADE ON DELETE CASCADE,
    character_name  text NOT NULL,
    CONSTRAINT  user_characters_pkey PRIMARY KEY (user_id, character_name),

    nsfw_description    text,
    nsfw_icons          text[],
    nsfw_summary        text,
    prefix              text,
    sfw_description     text NOT NULL,
    sfw_icons           text[],
    sfw_summary         text NOT NULL
);