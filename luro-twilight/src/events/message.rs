use luro_framework::{Luro, LuroContext};
use luro_model::{builders::EmbedBuilder, response::safe_truncate};
use twilight_model::gateway::payload::incoming::MessageCreate;

pub async fn create(framework: LuroContext, event: Box<MessageCreate>) -> anyhow::Result<()> {
    let characters = framework.database.user_fetch_characters(event.author.id).await?;
    let first_word = event.content.split(' ').next().unwrap_or("");

    // TODO: Actually get this from the database
    let mut user_character = None;
    for character in characters {
        if let Some(prefix) = &character.prefix {
            if prefix == first_word {
                user_character = Some(character);
            }
        }
    }

    if let Some(character) = user_character {
        let _ = framework.twilight_client.delete_message(event.channel_id, event.id).await;
        let channel = framework.database.channel_fetch(event.channel_id).await?;
        let proxied_message = event.content.replace(first_word, "");

        // Attempt to first send as a webhook
        if let Ok(webhook) = framework.get_webhook(channel.id).await {
            if let Some(token) = webhook.token {
                if let Some(data) = &event.reference {
                    if let Some(message_id) = data.message_id {
                        if let Ok(mut message) = framework.database.message_fetch(message_id, data.channel_id).await {
                            safe_truncate(&mut message.content, 150);
                            let mut embed = framework.default_embed().await;
                            let description = format!("[Replying to:]({}) {}", message.link(), message.content);

                            embed
                                .author(|a| a.name(message.author.name()).icon_url(message.author.avatar_url()))
                                .description(description);

                            let response = framework
                                .twilight_client
                                .execute_webhook(webhook.id, &token)
                                .embeds(&vec![embed.into()])
                                .username(&format!("{} [{}]", character.name, event.author.name))
                                .content(&proxied_message)
                                .avatar_url(match channel.nsfw.unwrap_or_default() {
                                    true => character.nsfw_icon.as_ref().unwrap_or(&character.sfw_icon),
                                    false => &character.sfw_icon,
                                })
                                .await;

                            if response.is_ok() {
                                return Ok(());
                            }
                        }
                    }
                }

                let response = framework
                    .twilight_client
                    .execute_webhook(webhook.id, &token)
                    .username(&format!("{} [{}]", character.name, event.author.name))
                    .content(&proxied_message)
                    .avatar_url(match channel.nsfw.unwrap_or_default() {
                        true => character.nsfw_icon.as_ref().unwrap_or(&character.sfw_icon),
                        false => &character.sfw_icon,
                    })
                    .await;

                if response.is_ok() {
                    return Ok(());
                }
            }
        }

        let mut embed = EmbedBuilder::default();
        embed
            .author(|a| {
                a.name(format!(
                    "{} - [{}]",
                    character.nickname.unwrap_or(character.name),
                    event.author.name
                ))
                .icon_url(match channel.nsfw.unwrap_or_default() {
                    true => character.nsfw_icon.as_ref().unwrap_or(&character.sfw_icon),
                    false => &character.sfw_icon,
                })
            })
            .colour(character.colour.unwrap_or(framework.accent_colour()))
            .description(proxied_message);

        framework
            .twilight_client
            .create_message(event.channel_id)
            .embeds(&vec![embed.into()])
            .reply(event.id)
            .await?;
    }

    Ok(())
}
