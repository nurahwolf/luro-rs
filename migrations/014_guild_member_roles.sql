-- DROP TABLE guild_roles;
CREATE TABLE IF NOT EXISTS guild_member_roles (
  user_id   bigint references users(user_id) ON UPDATE CASCADE ON DELETE CASCADE,
  role_id     bigint references guilds(guild_id) ON UPDATE CASCADE ON DELETE CASCADE,
  CONSTRAINT  guild_member_roles_pkey PRIMARY KEY (user_id, role_id)
);