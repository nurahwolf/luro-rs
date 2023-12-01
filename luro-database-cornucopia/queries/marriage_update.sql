--! marriage_update
INSERT INTO user_marriages (
        divorced,
        proposee_id,
        proposer_id,
        reason,
        rejected
    )
VALUES (
        :divorced,
        :proposer_id,
        :proposee_id,
        :reason,
        :rejected
    ) ON CONFLICT (proposer_id, proposee_id) DO
UPDATE
SET divorced = :divorced,
    proposer_id = :proposer_id,
    proposee_id = :proposee_id,
    reason = :reason,
    rejected = :rejected;