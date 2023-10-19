use luro_database::DbUserMarriage;
use luro_framework::{CommandInteraction, InteractionTrait, Luro, LuroCommand};
use luro_model::COLOUR_DANGER;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

use super::buttons;

#[derive(CommandModel, CreateCommand)]
#[command(name = "divorce", desc = "It's over.")]
pub struct Divorce {
    /// Who no longer deserves your love?
    pub user: ResolvedUser,
    /// Why do they no longer desrve your love?
    reason: String,
}

impl LuroCommand for Divorce {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let proposer = ctx.fetch_user(&ctx.author_id()).await?;
        let proposee = ctx.fetch_user(&self.user.resolved.id).await?;

        ctx.database
            .update_marriage(DbUserMarriage {
                proposer_id: proposer.user_id,
                proposee_id: proposee.user_id,
                divorced: true,
                rejected: false,
                reason: self.reason.clone(),
            })
            .await?;

        ctx.respond(|response| {
            response
                .embed(|embed| {
                    embed
                        .colour(COLOUR_DANGER)
                        .title(format!(
                            "{} has terminated their marriage with {}!",
                            proposer.name(),
                            proposee.name()
                        ))
                        .thumbnail(|t| t.url(proposer.avatar()))
                        .create_field("Their Reason", &self.reason, false)
                        .create_field("Approvers", "None!", false)
                        .create_field("Disapprovers", "None!", false)
                })
                .content(format!("Bad news <@{}>...", &self.user.resolved.id))
                .add_components(buttons())
        })
        .await
    }
}
