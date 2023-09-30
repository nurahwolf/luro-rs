use std::fmt::Write;

use async_trait::async_trait;
use luro_framework::{Framework, command::LuroCommandTrait, InteractionCommand, LuroInteraction};
use luro_model::database_driver::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};



#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "guilds", desc = "Information about all guilds")]
pub struct Guilds {
    /// Optionally include the guild ID
    show_id: Option<bool>,
}
#[async_trait]
impl LuroCommandTrait for Guilds {
    async fn handle_interaction(
        ctx: Framework,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        let mut guilds = String::new();
        for guild in ctx.cache.iter().guilds() {
            if data.show_id.unwrap_or_default() {
                writeln!(guilds, "{} - <#{1}> - {1}", guild.name(), guild.id())?
            } else {
                writeln!(guilds, "{} - <#{}>", guild.name(), guild.id())?
            }
        }

        let accent_colour = interaction.accent_colour(&ctx).await;
        interaction.respond(&ctx, |r| {
            r.embed(|embed| {
                embed
                    .title("All the guilds that I am in")
                    .description(guilds)
                    .colour(accent_colour)
            })
        })
        .await
    }
}
