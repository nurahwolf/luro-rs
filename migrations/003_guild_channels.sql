-- DROP TABLE guild_channels;
CREATE TABLE IF NOT EXISTS guild_channels (
    channel_id  bigint NOT NULL PRIMARY KEY,
    deleted     boolean NOT NULL DEFAULT false
);