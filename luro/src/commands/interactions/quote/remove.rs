use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "remove", desc = "Remove a particular quote (Owner Only)!")]
pub struct Remove {
    /// The quote to remove
    id: i64,
}

impl luro_framework::LuroCommand for Remove {
    async fn interaction_command(self, ctx: luro_framework::CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let mut quotes = ctx.database.get_quotes().await?;
        let id = usize::try_from(data.id).unwrap();

        let quote = match quotes.remove(&id) {
            Some(quote) => quote,
            None => {
                return interaction
                    .respond(&ctx, |r| r.content("That quote is not present!").ephemeral())
                    .await
            }
        };
        let user = ctx.database.get_user(&quote.author).await?;

        ctx.database.save_quotes(quotes).await?;

        let accent_colour = interaction.accent_colour(&ctx).await;
        interaction
            .respond(&ctx, |response| {
                response.embed(|embed| {
                    embed.colour(accent_colour).description(quote.content).author(|author| {
                        author
                            .name(format!("{} - Quote {id} Removed", user.name()))
                            .icon_url(user.avatar());
                        match quote.guild_id {
                            Some(guild_id) => author.url(format!(
                                "https://discord.com/channels/{guild_id}/{}/{}",
                                quote.channel_id, quote.id
                            )),
                            None => author.url(format!("https://discord.com/channels/{}/{}", quote.channel_id, quote.id)),
                        }
                    })
                })
            })
            .await
    }
}
