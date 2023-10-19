use anyhow::Context;
use luro_database::DbUserMarriage;
use luro_framework::{CommandInteraction, InteractionTrait, Luro, LuroCommand};
use rand::{seq::SliceRandom, thread_rng};
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

use super::{buttons, MARRIAGE_REASONS};

#[derive(CommandModel, CreateCommand)]
#[command(name = "someone", desc = "Propose to someone! So lucky, aww~")]
pub struct Someone {
    /// Set this if you want to marry someone!
    pub marry: ResolvedUser,
    /// The reason you wish to marry them!
    reason: Option<String>,
}

impl LuroCommand for Someone {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let proposer = ctx.get_user(&ctx.author_id()).await?;
        let accent_colour = ctx.accent_colour().await;

        let reason = self.reason.unwrap_or(
            MARRIAGE_REASONS
                .choose(&mut thread_rng())
                .context("Expected to be able to choose a random reason")?
                .replace("<user>", &format!("<@{}>", &self.marry.resolved.id))
                .replace("<author>", &format!("<@{}>", &proposer.id)),
        );

        ctx.database
            .update_marriage(DbUserMarriage {
                proposer_id: proposer.id.get() as i64,
                proposee_id: self.marry.resolved.id.get() as i64,
                divorced: false,
                rejected: false,
                reason: reason.clone(),
            })
            .await?;

        ctx.respond(|response| {
            response
                .embed(|embed| {
                    embed
                        .colour(accent_colour)
                        .title(format!("{} has proposed!", proposer.name()))
                        .thumbnail(|t| t.url(proposer.avatar()))
                        .create_field("Their Reason", &reason, false)
                        .create_field("Approvers", "None!", false)
                        .create_field("Disapprovers", "None!", false)
                })
                .content(format!("<@{}>", &self.marry.resolved.id))
                .add_components(buttons())
        })
        .await
    }
}
