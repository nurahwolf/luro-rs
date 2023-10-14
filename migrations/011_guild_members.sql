DROP TABLE guild_members;
CREATE TABLE IF NOT EXISTS guild_members (
  user_id     bigint references users(user_id) ON UPDATE CASCADE ON DELETE CASCADE,
  guild_id    bigint references guilds(guild_id) ON UPDATE CASCADE ON DELETE CASCADE,
  CONSTRAINT  guild_members_pkey PRIMARY KEY (guild_id, user_id),

  avatar                        jsonb,
  boosting_since                TIMESTAMPTZ,
  communication_disabled_until  TIMESTAMPTZ,
  deafened                      boolean NOT NULL,
  flags                         int NOT NULL,
  muted                         boolean NOT NULL,
  nickname                      text,
  pending                       bool NOT NULL
);