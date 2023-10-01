CREATE TYPE message_source AS ENUM (
    'MESSAGE', 'CUSTOM', 'CACHED_MESSAGE', 'MESSAGE_UPDATE', 'MESSAGE_DELETE', 'MESSAGE_CREATE', 'NONE'
);

CREATE TABLE IF NOT EXISTS messages (
    activity jsonb,
    application jsonb,
    application_id INT8,
    attachments jsonb,
    author jsonb NOT NULL,
    channel_id INT8 NOT NULL,
    components jsonb,
    content text,
    deleted boolean,
    edited_timestamp TIMESTAMPTZ,
    embeds jsonb,
    flags jsonb,
    guild_id INT8,
    id INT8 NOT NULL PRIMARY KEY,
    interaction jsonb,
    kind jsonb NOT NULL,
    member jsonb,
    mention_channels jsonb,
    mention_everyone boolean,
    mention_roles INT8[],
    mentions jsonb,
    message_updates jsonb,
    pinned boolean,
    reactions jsonb,
    reference jsonb,
    referenced_message jsonb,
    role_subscription_data jsonb,
    source message_source NOT NULL,
    sticker_items jsonb,
    thread jsonb,
    timestamp TIMESTAMPTZ NOT NULL,
    tts boolean,
    webhook_id INT8
);