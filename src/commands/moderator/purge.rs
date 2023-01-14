use poise::serenity_prelude::User;

use crate::{Context, Error};

/// Purge X messages. Note that up to 100 messages can be got at a time! - Moderator only
#[poise::command(
    prefix_command,
    slash_command,
    category = "Moderation",
    required_bot_permissions = "MANAGE_MESSAGES",
    required_permissions = "MANAGE_MESSAGES"
)]
pub async fn purge(
    ctx: Context<'_>,
    #[description = "Number of messages to delete between 1 - 100"] num: usize,
    #[description = "Purge from a particular user"] user: Option<User>
) -> Result<(), Error> {
    // Delete from one particular user
    if let Some(user) = user {
        let messages_to_delete = ctx.channel_id().messages(ctx, |m| m.limit(100)).await?.into_iter().filter(|filter| filter.author == user);

        let messages_vector: Vec<_> = messages_to_delete.clone().collect();
        if messages_vector.is_empty() {
            ctx.say("No messages found to delete, sorry").await?;
        } else if messages_vector.len() == 1 {
            ctx.channel_id().delete_message(ctx, messages_vector.first().unwrap().id).await?;
            ctx.say(format!("Deleted 1 message by user {user} 👌")).await?;
        } else {
            ctx.channel_id().delete_messages(ctx, messages_to_delete).await?;
            ctx.say(format!("Deleted {} messages by user {} 👌", messages_vector.len(), user)).await?;
        }
        return Ok(());
    }

    let number_of_messages: u64 = num.try_into().unwrap_or(1);
    let messages_to_delete = ctx.channel_id().messages(ctx, |m| m.limit(number_of_messages)).await?.into_iter();
    let messages_vector: Vec<_> = messages_to_delete.clone().collect();

    if num == 0 || num >= 100 {
        ctx.say("Select 1 to 100 ;)").await?;
    } else if num == 1 {
        match messages_to_delete.last() {
            Some(message) => {
                ctx.channel_id().delete_message(ctx, message).await?;
                ctx.say("Deleted 1 message! 👌").await?;
            }
            None => {
                ctx.say("Failed to delete the message :(").await?;
            }
        }
    } else if num <= 100 {
        ctx.channel_id().delete_messages(ctx, messages_to_delete).await?;
        ctx.say(format!("Deleted {} messages 👌", messages_vector.len())).await?;
    }

    Ok(())
}
