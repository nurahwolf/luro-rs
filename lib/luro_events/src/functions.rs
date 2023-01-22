use luro_core::Data;
use luro_sled::get_discord_message;
use luro_utilities::guild_accent_colour;
use poise::serenity_prelude::{ChannelId, Context, CreateEmbed, CreateMessage, GuildChannel};

pub async fn deleted_message_formatted<'a>(
    ctx: &'a Context,
    alert_channel: &'a GuildChannel,
    data: &'a Data,
    message_id: u64,
    channel_id: &'a ChannelId,
    hide: bool
) -> CreateMessage<'a> {
    let accent_colour = data.config.read().await.accent_colour;
    let mut embed = CreateEmbed::default();

    match get_discord_message(&data.database, message_id) {
        Ok(luro_message) => {
            let message_resolved = ctx.http.get_message(luro_message.channel_id, luro_message.message_id).await;
            let mut embed = CreateEmbed::default();

            embed.title("Message Deleted");
            embed.description(&luro_message.message_content);
            embed.color(guild_accent_colour(accent_colour, alert_channel.guild(&ctx)));
            embed.footer(|footer| footer.text("This message was fetched from the database, so most likely no longer exists"));

            if let Ok(message_user) = &ctx.http.get_user(luro_message.user_id).await {
                embed.author(|author| {
                    author
                        .name(&message_user.name)
                        .icon_url(&message_user.avatar_url().unwrap_or_default())
                });
            }

            if !hide {
                embed.field("Message ID", &luro_message.message_id, true);
                embed.field("Channel ID", &luro_message.channel_id, true);
                embed.field("User ID", &luro_message.user_id, true);

                if let Some(guild_id) = &luro_message.guild_id && message_resolved.is_err() {
                embed.field("Guild ID", guild_id, true);
            }
            }

            let mut message = CreateMessage::default();
            message.add_embed(|e| {
                *e = embed;
                e
            });
            message
        }
        Err(_) => {
            embed.title("Message Deleted").description(format!(
                "The message with ID {} just got deleted in channel {}!",
                message_id, channel_id
            ));
            let mut message = CreateMessage::default();
            message.add_embed(|e| {
                *e = embed;
                e
            });
            message
        }
    }
}
