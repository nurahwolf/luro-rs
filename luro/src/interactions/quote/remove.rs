use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "remove", desc = "Remove a particular quote (Owner Only)!")]
pub struct Remove {
    /// The quote to remove
    id: i64
}

impl LuroCommand for Remove {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let mut quotes = ctx.framework.database.get_quotes().await?;
        let id = usize::try_from(self.id).unwrap();

        let quote = match quotes.remove(&id) {
            Some(quote) => quote,
            None => return ctx.respond(|r| r.content("That quote is not present!").ephemeral()).await
        };
        let user = ctx
            .framework
            .database
            .get_user(&quote.author, &ctx.framework.twilight_client)
            .await?;

        ctx.framework.database.save_quotes(quotes).await?;

        let accent_colour = ctx.accent_colour().await;
        ctx.respond(|response| {
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
                        None => author.url(format!("https://discord.com/channels/{}/{}", quote.channel_id, quote.id))
                    }
                })
            })
        })
        .await
    }
}
