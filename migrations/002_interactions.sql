CREATE TABLE IF NOT EXISTS interactions (
    application_id      INT8 NOT NULL,
    interaction_id      INT8 NOT NULL,
    message_id          INT8,
    kind                BYTEA NOT NULL,
    token               VARCHAR(64) NOT NULL,
    data                BYTEA NOT NULL,
    PRIMARY KEY (interaction_id)
);