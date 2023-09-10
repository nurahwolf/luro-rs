use std::fmt::Write;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

use crate::interaction::LuroSlash;
use luro_model::database::drivers::LuroDatabaseDriver;

use crate::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand,)]
#[command(name = "punishments", desc = "Information about a user's punishments")]
pub struct Punishments {
    /// The user to get
    user: ResolvedUser,
}

impl LuroCommand for Punishments {
    async fn run_command<D: LuroDatabaseDriver,>(self, ctx: LuroSlash<D,>,) -> anyhow::Result<(),> {
        let punished_user = ctx.framework.database.get_user(&self.user.resolved.id,).await?;
        let mut warnings = String::new();

        for (warning, id,) in &punished_user.warnings {
            writeln!(warnings, "- ID <@{id}>: {warning}")?
        }

        if warnings.is_empty() {
            return ctx.respond(|r| r.content("User has no warnings!",).ephemeral(),).await;
        }

        let colour = ctx.accent_colour().await;
        ctx.respond(|response| {
            response.embed(|embed| {
                embed.colour(colour,).description(warnings,).author(|author| {
                    author
                        .name(format!("{}'s warnings", punished_user.member_name(&ctx.interaction.guild_id)),)
                        .icon_url(punished_user.avatar(),)
                },)
            },)
        },)
            .await
    }
}
