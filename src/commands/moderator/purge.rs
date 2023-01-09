use crate::{Context, Error};

/// Purge X messages - Moderator only
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderation",
    required_bot_permissions = "MANAGE_MESSAGES",
    ephemeral,
    required_permissions = "MANAGE_MESSAGES"
)]
pub async fn cleanup(ctx: Context<'_>, #[description = "Number of messages to delete between 1 - 100"] num: usize) -> Result<(), Error> {
    if num == 0 {
        ctx.say("Select 1 to 100 ;)").await?;
    } else if num == 1 {
        match ctx.channel_id().messages(ctx, |m| m.limit(1)).await?.into_iter().last() {
            Some(message) => {
                ctx.channel_id().delete_message(ctx, message).await?;
                ctx.say("👌").await?;
            }
            None => {
                ctx.say("Failed to find message to delete :(").await?;
            }
        }
    } else if num <= 100 {
        let messages_to_delete = ctx.channel_id().messages(ctx, |m| m.limit(100)).await?.into_iter().take(num);

        ctx.channel_id().delete_messages(ctx, messages_to_delete).await?;
        ctx.say("👌").await?;
    } else {
        ctx.say("Blame Discord's API for not letting me delete that number, sorry.").await?;
    }

    Ok(())
}
