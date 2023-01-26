use std::sync::Arc;

use luro_utilities::guild_accent_colour;
use poise::serenity_prelude::{Cache, CreateEmbed, Guild, Message};

pub fn embed(
    message: &Message,
    accent_colour: [u8; 3],
    guild: Option<Guild>,
    cursor: usize,
    hide: bool,
    cache: Arc<Cache>
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
        .footer(|footer| footer.text(format!("Fav ID: {cursor}")));
    if let Some(attachment) = message.attachments.first() {
        embed.image(&attachment.proxy_url);
    }

    if !hide {
        embed.field("Message ID", message.id, true);
        if let Some(guild_id) = message.guild_id {
            embed.field("Guild ID", guild_id, true);
        }
        embed.field("Author", format!("{} (ID: {})", message.author, message.author.id), true);
    }

    if let Some(guild) = message.guild(cache) {
        embed.footer(|footer| footer.icon_url(guild.icon_url().unwrap_or_default()).text(guild.name));
    };

    embed
}
