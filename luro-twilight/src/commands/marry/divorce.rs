use luro_database::DbUserMarriage;
use luro_framework::{command::ExecuteLuroCommand, interactions::InteractionTrait, CommandInteraction, Luro};
use luro_model::COLOUR_DANGER;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

use super::buttons;

#[derive(CommandModel, CreateCommand)]
#[command(name = "divorce", desc = "It's over.")]
pub struct Divorce {
    /// Who no longer deserves your love?
    pub user: ResolvedUser,
    /// Why do they no longer desrve your love?
    reason: String
}

impl ExecuteLuroCommand for Divorce {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let proposer = ctx.get_user(&ctx.author_id()).await?;
        let proposee = ctx.get_user(&self.user.resolved.id).await?;

        ctx.database.update_marriage(DbUserMarriage {
            proposer_id: proposer.id.get() as i64,
            proposee_id: self.user.resolved.id.get() as i64,
            divorced: true,
            rejected: false,
            reason: self.reason.clone(),
        }).await?;

        ctx.respond(|response| {
            response
                .embed(|embed| {
                    embed
                        .colour(COLOUR_DANGER)
                        .title(format!("{} has terminated their marriage with {}!", proposer.name(), proposee.name()))
                        .thumbnail(|t|t.url(proposer.avatar()))
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
