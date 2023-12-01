SELECT
    activity as "activity: Json<MessageActivity>",
    application_id,
    application as "application: Json<MessageApplication>",
    attachments as "attachments: Json<Vec<Attachment>>",
    author as "author: Json<User>",
    author_id,
    messages.channel_id,
    components as "components: Json<Vec<Component>>",
    content,
    deleted,
    edited_timestamp,
    embeds as "embeds: Json<Vec<Embed>>",
    flags as "flags: Json<MessageFlags>",
    guild_id,
    messages.message_id,
    interaction as "interaction: Json<MessageInteraction>",
    kind as "kind: Json<MessageType>",
    mention_channels as "mention_channels: Json<Vec<ChannelMention>>",
    mention_everyone,
    mention_roles as "mention_roles: Vec<i64>",
    mentions as "mentions: Json<Vec<Mention>>",
    pinned,
    reactions as "reactions: Json<Vec<Reaction>>",
    reference as "reference: Json<MessageReference>",
    referenced_message as "referenced_message: Json<Box<Message>>",
    role_subscription_data as "role_subscription_data: Json<RoleSubscriptionData>",
    source as "source: DbMessageSource",
    sticker_items as "sticker_items: Json<Vec<MessageSticker>>",
    thread as "thread: Json<Channel>",
    timestamp,
    member as "member: Json<PartialMember>",
    tts,
    webhook_id,
    message_updates as "message_updates: Json<Vec<MessageUpdate>>",
    quotes.id,
    quotes.nsfw
FROM quotes
    INNER JOIN messages ON messages.message_id = quotes.message_id