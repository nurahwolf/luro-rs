-- SELECT 
--     COUNT(*)
-- FROM 
--     messages

DROP TABLE interactions;
DROP TYPE interaction_kind;
-- CREATE TYPE interaction_kind AS ENUM ('PING', 'APPLICATION_COMMAND', 'MESSAGE_COMPONENT', 'APPLICATION_COMMAND_AUTOCOMPLETE', 'MODAL_SUBMIT', 'UNKNOWN');
-- CREATE TABLE IF NOT EXISTS interactions (
--     app_permissions     BIGINT NOT NULL,
--     application_id      BIGINT NOT NULL,
--     channel             BIGINT NOT NULL,
--     data                JSONB,
--     guild_id            BIGINT,
--     guild_locale        TEXT,
--     interaction_id      BIGINT NOT NULL PRIMARY KEY,
--     kind                interaction_kind NOT NULL,
--     locale              TEXT,
--     member              BIGINT,
--     message_id          BIGINT,
--     token               TEXT NOT NULL,
--     user                BIGINT
-- );