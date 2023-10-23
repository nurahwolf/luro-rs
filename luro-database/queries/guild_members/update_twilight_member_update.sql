INSERT INTO guild_members (
    boosting_since,
    communication_disabled_until,
    guild_id,
    joined_at,
    member_avatar,
    nickname,
    pending,
    user_id
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
ON CONFLICT (guild_id, user_id)
    DO UPDATE SET
        boosting_since = $1,
        communication_disabled_until = $2,
        joined_at = $4,
        member_avatar = $5,
        nickname = $6,
        pending = $7