use std::fmt::Write;

use luro_framework::{CommandInteraction, Luro, LuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "punishments", desc = "Information about a user's punishments")]
pub struct Punishments {
    /// The user to get
    user: ResolvedUser,
}

impl LuroCommand for Punishments {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let punished_user = ctx.fetch_user(&self.user.resolved.id).await?;
        let mut warnings = String::new();

        for (warning, id) in &punished_user.warnings {
            writeln!(warnings, "- ID <@{id}>: {warning}")?
        }

        if warnings.is_empty() {
            return ctx.respond(|r| r.content("User has no warnings!").ephemeral()).await;
        }

        let colour = ctx.accent_colour();
        ctx.respond(|response| {
            response.embed(|embed| {
                embed.colour(colour).description(warnings).author(|author| {
                    author
                        .name(format!("{}'s warnings", punished_user.name()))
                        .icon_url(punished_user.avatar())
                })
            })
        })
        .await
    }
}
