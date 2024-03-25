INSERT INTO user_marriage_approvals (
        approve,
        disapprove,
        proposee_id,
        proposer_id,
        user_id
    )
VALUES ($1, $2, $3, $4, $5) ON CONFLICT (proposer_id, proposee_id, user_id) DO
UPDATE
SET approve = $1,
    disapprove = $2
RETURNING approve,
    disapprove,
    proposee_id,
    proposer_id,
    user_id