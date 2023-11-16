CREATE TABLE IF NOT EXISTS guild_member_roles (
    user_id bigint NOT NULL,
    role_id bigint NOT NULL,
    guild_id bigint NOT NULL
);