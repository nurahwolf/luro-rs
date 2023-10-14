-- DROP TABLE guild_channels;
CREATE TABLE IF NOT EXISTS guild_channels (
    channel_id  bigint NOT NULL PRIMARY KEY,
    guild_id    bigint references guilds(guild_id),

    deleted     boolean NOT NULL DEFAULT false
);