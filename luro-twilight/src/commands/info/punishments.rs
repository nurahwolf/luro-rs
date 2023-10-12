use std::fmt::Write;

use luro_framework::{command::ExecuteLuroCommand, interactions::InteractionTrait, CommandInteraction, Luro};
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "punishments", desc = "Information about a user's punishments")]
pub struct Punishments {
    /// The user to get
    user: ResolvedUser,
}

impl ExecuteLuroCommand for Punishments {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let punished_user = ctx.get_user(&self.user.resolved.id).await?;
        let mut warnings = String::new();

        for (warning, id) in &punished_user.warnings {
            writeln!(warnings, "- ID <@{id}>: {warning}")?
        }

        if warnings.is_empty() {
            return ctx.respond(|r| r.content("User has no warnings!").ephemeral()).await;
        }

        let colour = ctx.accent_colour().await;
        ctx.respond(|response| {
            response.embed(|embed| {
                embed.colour(colour).description(warnings).author(|author| {
                    author
                        .name(format!("{}'s warnings", punished_user.member_name(&ctx.guild_id)))
                        .icon_url(punished_user.avatar())
                })
            })
        })
        .await
    }
}
