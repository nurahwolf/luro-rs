DO $$ BEGIN
    CREATE TYPE message_source AS ENUM ('MESSAGE', 'CUSTOM', 'CACHED_MESSAGE', 'MESSAGE_UPDATE', 'MESSAGE_DELETE', 'MESSAGE_CREATE', 'NONE');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS messages (
    activity JSONB,
    application JSONB,
    application_id BIGINT REFERENCES applications(application_id),
    attachments JSONB,
    author JSONB NOT NULL, -- Save data on what the user looked like at time of posting (such as avatar)
    author_id BIGINT REFERENCES users(user_id), -- Used to get information on the author NOW
    channel_id BIGINT NOT NULL REFERENCES channels(channel_id),
    components JSONB,
    content TEXT,
    deleted BOOLEAN,
    edited_timestamp TIMESTAMPTZ,
    embeds JSONB,
    flags JSONB,
    guild_id BIGINT REFERENCES guilds(guild_id),
    interaction JSONB,
    kind JSONB NOT NULL,
    member JSONB,
    mention_channels JSONB,
    mention_everyone BOOLEAN,
    mention_roles BIGINT references roles(role_id),
    mentions JSONB,
    message_id BIGINT NOT NULL PRIMARY KEY,
    message_updates JSONB,
    pinned BOOLEAN,
    reactions JSONB,
    reference JSONB,
    referenced_message JSONB,
    role_subscription_data JSONB,
    source message_source NOT NULL,
    sticker_items JSONB,
    thread JSONB,
    timestamp TIMESTAMPTZ NOT NULL,
    tts BOOLEAN,
    webhook_id BIGINT references webhooks(webhook_id)
);