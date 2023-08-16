use luro_model::{luro_message::LuroMessage, slash_user::SlashUser};
use tracing::debug;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::ChannelMarker, Id};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "add", desc = "Save what someone said!")]
pub struct Add {
    /// The message ID to save
    id: String,
    /// Only set this if Luro can't find the message in the cache.
    channel: Option<Id<ChannelMarker>>
}

impl LuroCommand for Add {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let interaction = &ctx.interaction;
        let channel_id = self.channel.unwrap_or(interaction.channel.as_ref().unwrap().id);
        let accent_colour = ctx.accent_colour().await;
        let id = Id::new(self.id.parse()?);
        let quoted_user;

        let mut quote = match ctx.framework.twilight_client.message(channel_id, id).await {
            Ok(message) => {
                let message = message.model().await?;
                quoted_user = message.author.clone();
                LuroMessage::from(message)
            }
            Err(_) => {
                debug!("Failed to get message via twilight, so lookign in the cache");
                match ctx.framework.twilight_cache.message(id) {
                    Some(message) => {
                        let mut quote = LuroMessage::from(message.clone());
                        // Add some more stuff that is not in the cache
                        if let Some(user) = ctx.framework.twilight_cache.user(message.author()) {
                            quoted_user = user.clone();
                            quote.add_user(&user.clone());
                        } else {
                            let user = ctx.framework.twilight_client.user(message.author()).await?.model().await?;
                            quoted_user = user.clone();
                            quote.add_user(&user.clone());
                        }
                        quote
                    },
                    None => return ctx
                    .respond(|r| {
                        r.content("Sorry! Could not find that message. You sure you gave me the right ID?\nTry specifying the exact channel with the optional parameter if I'm struggling. If I still can't, it's probably because I don't have access to the channel.")
                            .ephemeral()
                    })
                    .await,
                }
            }
        };

        if let Some(guild_id) = interaction.guild_id {
            if let Ok(member) = ctx.framework.twilight_client.guild_member(guild_id, quoted_user.id).await {
                quote.add_member(&quoted_user, &member.model().await?, &guild_id);
            }
        }

        let slash_user = SlashUser::from(quoted_user);

        let local_quotes = ctx.framework.database.get_quotes().await?;
        let local_quote_id = local_quotes.len();

        ctx.framework.database.save_quote(local_quote_id, quote.clone()).await?;

        ctx.respond(|response| {
            response.embed(|embed| {
                embed
                    .colour(accent_colour)
                    .description(quote.content.unwrap_or_default())
                    .author(|author| {
                        author
                            .name(format!("{} - Quote {local_quote_id}", slash_user.name))
                            .icon_url(slash_user.avatar);
                        match quote.guild_id {
                            Some(guild_id) => author.url(format!(
                                "https://discord.com/channels/{guild_id}/{}/{}",
                                quote.channel_id, quote.id
                            )),
                            None => author.url(format!("https://discord.com/channels/{}/{}", quote.channel_id, quote.id))
                        }
                    })
            })
        })
        .await
    }
}
