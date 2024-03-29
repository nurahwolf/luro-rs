INSERT INTO guild_members (
    boosting_since,
    communication_disabled_until,
    deafened,
    guild_id,
    joined_at,
    member_avatar,
    member_flags,
    muted,
    nickname,
    user_id
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
ON CONFLICT (guild_id, user_id)
    DO UPDATE SET
        boosting_since = $1,
        communication_disabled_until = $2,
        deafened = $3,
        joined_at = $5,
        member_avatar = $6,
        member_flags = $7,
        muted = $8,
        nickname = $9
