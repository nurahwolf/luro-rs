DO $$ BEGIN
    CREATE TYPE interaction_kind AS ENUM ('PING', 'APPLICATION_COMMAND', 'MESSAGE_COMPONENT', 'APPLICATION_COMMAND_AUTOCOMPLETE', 'MODAL_SUBMIT', 'UNKNOWN');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- DROP TABLE interactions;
CREATE TABLE IF NOT EXISTS interactions (
    FOREIGN KEY         (guild_id, user_id) references guild_members (guild_id, user_id),
    guild_id            bigint,
    interaction_id      bigint PRIMARY KEY,
    user_id             BIGINT NOT NULL references users(user_id),

    app_permissions     BIGINT NOT NULL,
    application_id      BIGINT NOT NULL references applications(application_id),
    channel_id          BIGINT NOT NULL references channels(channel_id),
    data                JSONB,
    guild_locale        TEXT,
    kind                interaction_kind NOT NULL,
    locale              TEXT,
    message_id          BIGINT references messages(message_id),
    token               TEXT NOT NULL
);