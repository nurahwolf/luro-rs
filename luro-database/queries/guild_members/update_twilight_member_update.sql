INSERT INTO guild_members (
    boosting_since,
    communication_disabled_until,
    deafened,
    guild_id,
    joined_at,
    member_avatar,
    muted,
    nickname,
    pending
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
ON CONFLICT (user_id)
    DO UPDATE SET
        boosting_since = $1,
        communication_disabled_until = $2,
        deafened = $3,
        joined_at = $5,
        member_avatar = $6,
        muted = $7,
        nickname = $8,
        pending = $9