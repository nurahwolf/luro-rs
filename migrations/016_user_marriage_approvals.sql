DROP TABLE user_marriage_approvals;
CREATE TABLE IF NOT EXISTS user_marriage_approvals (
    CONSTRAINT      user_marriage_approvals_pkey PRIMARY KEY (user_id, proposee_id, proposer_id),
    FOREIGN KEY         (proposer_id, proposee_id) references user_marriages (proposer_id, proposee_id),
    proposee_id            bigint NOT NULL,
    proposer_id            bigint NOT NULL,
    user_id         bigint NOT NULL references users(user_id) ON UPDATE CASCADE ON DELETE CASCADE,

    approve          boolean NOT NULL default false,
    disapprove       boolean NOT NULL default false
);