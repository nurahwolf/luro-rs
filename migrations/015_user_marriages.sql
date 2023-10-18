DROP TABLE user_marriages;
CREATE TABLE IF NOT EXISTS user_marriages (
    proposer_id     bigint NOT NULL references users(user_id) ON UPDATE CASCADE ON DELETE CASCADE,
    proposee_id     bigint NOT NULL references users(user_id) ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT      user_marriages_pkey PRIMARY KEY (proposer_id, proposee_id),

    active          boolean NOT NULL default true,
    rejected        boolean NOT NULL default false,
    reason          text NOT NULL
);