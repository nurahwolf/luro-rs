SELECT
    app_permissions,
    application_id,
    channel_id,
    data as "data: Json<InteractionData>",
    guild_id,
    guild_locale,
    interaction_id,
    kind as "kind: DbInteractionKind",
    member as "member: Json<PartialMember>",
    locale,
    message_id,
    token,
    user_id
FROM
    interactions
WHERE
    interaction_id = $1