DROP TABLE guilds;
CREATE TABLE IF NOT EXISTS guilds (
    guild_id    bigint NOT NULL PRIMARY KEY,
    name        text NOT NULL,
    owner_id    bigint NOT NULL REFERENCES users(user_id),
    members     bigint REFERENCES guild_members(guild_id,user_id)
);