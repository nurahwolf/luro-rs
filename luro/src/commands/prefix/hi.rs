pub async fn cmd(framework: &crate::models::message_context::MessageContext) {
    let message_client = framework
        .gateway
        .twilight_client
        .create_message(framework.ctx.channel_id)
        .reply(framework.ctx.id);

    if let Err(why) = message_client.content("Yeah, hi or something I guess.").await {
        tracing::error!(?why, "prefix_command - Failed to send message");
    }
}
