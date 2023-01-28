use std::sync::Arc;

use luro_utilities::guild_accent_colour;
use poise::serenity_prelude::{Cache, CreateEmbed, Guild, Message};
use regex::Regex;

/// Formats an embed with some default settings. Set `extra_info` to true to show information on the message, author and guild
pub fn embed(
    message: &Message,
    accent_colour: [u8; 3],
    guild: Option<Guild>,
    cursor: usize,
    extra_info: bool,
    cache: Arc<Cache>,
    category: String
) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed
        .author(|author| {
            author
                .name(&message.author.name)
                .icon_url(&message.author.avatar_url().unwrap_or_default())
                .url(message.link())
        })
        .color(guild_accent_colour(accent_colour, guild))
        .description(&message.content)
        .footer(|footer| footer.text(format!("Fav ID: {cursor}    Category: {category}")));
    if let Some(attachment) = message.attachments.first() {
        embed.image(&attachment.proxy_url);
    }

    // If an image link is contained within the message, add the FIRST as an attachment
    if let Ok(regex) = Regex::new(r"(?:https://)\S*(?:gif|jpe?g|tiff?|png|webp|bmp)") {
        if let Some(image_match) = regex.find(&message.content) {
            embed.image(image_match.as_str());
        }
    }

    if extra_info {
        embed.field("Message ID", message.id, true);
        if let Some(guild_id) = message.guild_id {
            embed.field("Guild ID", guild_id, true);
        }
        embed.field("Author", format!("{} ({})", message.author, message.author.id), true);
    }

    if let Some(guild) = message.guild(cache) {
        embed.footer(|footer| footer.icon_url(guild.icon_url().unwrap_or_default()).text(guild.name));
    };

    embed
}
