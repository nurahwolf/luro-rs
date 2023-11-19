use luro_model::types::{CharacterProfile, CommandResponse, User};
use twilight_model::id::{marker::ChannelMarker, Id};

use crate::Luro;

pub async fn proxy_character<L: Luro + Sync>(
    author: &User,
    channel_id: Id<ChannelMarker>,
    character: CharacterProfile,
    ctx: L,
    message: String,
    nsfw: bool,
) -> anyhow::Result<CommandResponse> {
    let character_icon = match nsfw {
        true => character.nsfw_icon.unwrap_or(character.sfw_icon),
        false => character.sfw_icon,
    };

    // Attempt to first send as a webhook
    if let Ok(webhook) = ctx.get_webhook(channel_id).await {
        if let Some(token) = webhook.token {
            let response = ctx
                .twilight_client()
                .execute_webhook(webhook.id, &token)
                .username(&format!("{} [{}]", character.name, author.name()))
                .content(&message)
                .avatar_url(&character_icon)
                .await;

            if response.is_ok() {
                return Ok(CommandResponse::default());
            }
        }
    }

    Ok(CommandResponse::default())
}
