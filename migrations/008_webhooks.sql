CREATE TABLE IF NOT EXISTS channels (
    webhook_id BIGINT NOT NULL PRIMARY KEY,
    guild_id BIGINT REFERENCES guilds(guild_id)
);