DO $$ BEGIN
    CREATE TYPE interaction_kind AS ENUM ('PING', 'APPLICATION_COMMAND', 'MESSAGE_COMPONENT', 'APPLICATION_COMMAND_AUTOCOMPLETE', 'MODAL_SUBMIT', 'UNKNOWN');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS interactions (
    app_permissions     BIGINT NOT NULL,
    application_id      BIGINT NOT NULL REFERENCES applications(application_id),
    channel_id          BIGINT NOT NULL REFERENCES channels(channel_id),
    data                JSONB,
    guild_id            BIGINT REFERENCES guilds(guild_id),
    guild_locale        TEXT,
    interaction_id      BIGINT NOT NULL PRIMARY KEY,
    kind                interaction_kind NOT NULL,
    locale              TEXT,
    member_id           BIGINT NOT NULL REFERENCES users(user_id), -- TODO: Change this
    message_id          BIGINT REFERENCES messages(message_id),
    token               TEXT NOT NULL,
    user_id             BIGINT REFERENCES users(user_id)
);