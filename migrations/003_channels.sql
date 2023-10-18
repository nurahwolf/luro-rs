DROP TABLE channels;
CREATE TABLE IF NOT EXISTS channels (
    channel_id  bigint NOT NULL PRIMARY KEY,
    guild_id    bigint references guilds(guild_id),

    deleted     boolean NOT NULL DEFAULT false
);