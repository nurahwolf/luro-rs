use luro_core::Data;
use luro_sled::get_discord_message;
use luro_utilities::guild_accent_colour;
use poise::serenity_prelude::{ChannelId, Colour, Context, CreateEmbed, CreateMessage, GuildChannel, User};

pub async fn event_embed(accent_colour: Colour, event_author: Option<&User>, modified_user: Option<&User>) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.colour(accent_colour);

    if let Some(event_author) = event_author {
        embed.footer(|footer| {
            footer
                .text(format!("Action by: {}#{}", event_author.name, event_author.discriminator))
                .icon_url(event_author.avatar_url().unwrap_or_default())
        });
    };

    if let Some(modified_user) = modified_user {
        embed.author(|author| {
            author
                .name(format!("{}#{}", modified_user.name, modified_user.discriminator))
                .icon_url(modified_user.avatar_url().unwrap_or_default())
        });
        embed.thumbnail(modified_user.avatar_url().unwrap_or_default());
    };

    embed
}

pub async fn deleted_message_formatted<'a>(
    ctx: &'a Context,
    accent_colour: [u8; 3],
    alert_channel: &'a GuildChannel,
    data: &'a Data,
    message_id: u64,
    channel_id: &'a ChannelId,
    hide: bool
) -> CreateMessage<'a> {
    match get_discord_message(&data.database, message_id) {
        Ok(luro_message) => {
            let message_resolved = ctx.http.get_message(luro_message.channel_id, luro_message.message_id).await;
            let mut embed = match &ctx.http.get_user(luro_message.user_id).await {
                Ok(user) => event_embed(guild_accent_colour(accent_colour, alert_channel.guild(ctx)), Some(user), None).await,
                Err(_) => event_embed(guild_accent_colour(accent_colour, alert_channel.guild(ctx)), None, None).await
            };

            embed.title("Message Deleted");
            embed.description(&luro_message.message_content);
            embed.color(guild_accent_colour(accent_colour, alert_channel.guild(ctx)));
            embed.footer(|footer| footer.text("This message was fetched from the database, so most likely no longer exists"));

            if !hide {
                embed.field("Message ID", luro_message.message_id, true);
                embed.field("Channel ID", luro_message.channel_id, true);
                embed.field("User ID", luro_message.user_id, true);
            }

            let mut message = CreateMessage::default();
            message.add_embed(|e| {
                *e = embed;
                e
            });
            message
        }
        Err(_) => {
            let mut embed = CreateEmbed::default();
            embed
                .title("Message Deleted")
                .description(format!(
                    "The message with ID {message_id} just got deleted in channel {channel_id}!"
                ))
                .color(guild_accent_colour(accent_colour, alert_channel.guild(ctx)));
            let mut message = CreateMessage::default();
            message.add_embed(|e| {
                *e = embed;
                e
            });
            message
        }
    }
}
