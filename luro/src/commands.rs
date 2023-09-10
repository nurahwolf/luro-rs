use tracing::error;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{models::LuroWebhook, LuroFramework};

pub async fn message_handler(ctx: &LuroFramework, message: MessageCreate) -> anyhow::Result<()> {
    ctx.response_message_modified(&message.clone().into()).await?;
    let user_data = ctx.database.get_user(&message.author.id).await?;
    let first_word = message.content.split(' ').next().unwrap_or("");
    if let Some(character_name) = user_data.character_prefix.get(first_word) {
        let character = match user_data.characters.get(character_name) {
            Some(character) => character,
            None => return Ok(()),
        };
        let character_icon = match !character.icon.is_empty() {
            true => character.icon.clone(),
            false => user_data.avatar(),
        };

        ctx.twilight_client.delete_message(message.channel_id, message.id).await?;

        let luro_webhook = LuroWebhook::new(ctx.clone());
        let webhook = luro_webhook.get_webhook(message.channel_id).await?;
        let webhook_token = match webhook.token {
            Some(token) => token,
            None => match ctx.twilight_client.webhook(webhook.id).await?.model().await?.token {
                Some(token) => token,
                None => {
                    error!(
                        "I cannot setup a webhook in channel {} in response to message {}",
                        message.channel_id, message.id
                    );
                    return Ok(());
                }
            },
        };

        let proxied_message = message.content.replace(first_word, "");
        let mut embed = ctx.default_embed(&message.guild_id).await;
        embed.description(&proxied_message).author(|author| {
            author
                .name(&format!("{character_name} - Controlled by {}", user_data.name()))
                .icon_url(&character_icon)
        });

        ctx.twilight_client
            .execute_webhook(webhook.id, &webhook_token)
            .username(&user_data.member_name(&message.guild_id))
            .avatar_url(&user_data.avatar())
            .await?;
    }

    Ok(())
}
