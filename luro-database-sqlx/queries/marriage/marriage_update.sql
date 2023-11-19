INSERT INTO user_marriages (
        divorced,
        proposee_id,
        proposer_id,
        reason,
        rejected
    )
VALUES ($1, $2, $3, $4, $5) ON CONFLICT (proposer_id, proposee_id) DO
UPDATE
SET divorced = $1,
    proposer_id = $2,
    proposee_id = $3,
    reason = $4,
    rejected = $5