use crate::{Context, Error};

/// Purge X messages from the bot
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderation",
    required_bot_permissions = "MANAGE_MESSAGES",
    ephemeral
)]
pub async fn cleanup(
    ctx: Context<'_>,
    #[description = "Number of messages to delete"] num_messages: Option<usize>
) -> Result<(), Error> {
    let num_messages = num_messages.unwrap_or(5);

    let messages_to_delete = ctx
        .channel_id()
        .messages(ctx, |m| m.limit(100))
        .await?
        .into_iter()
        .filter(|msg| {
            if msg.author.id != ctx.framework().bot_id {
                return false;
            }
            if (*ctx.created_at() - *msg.timestamp).num_hours() >= 24 {
                return false;
            }
            true
        })
        .take(num_messages);

    ctx.channel_id().delete_messages(ctx, messages_to_delete).await?;

    ctx.say("👌").await?;

    Ok(())
}
