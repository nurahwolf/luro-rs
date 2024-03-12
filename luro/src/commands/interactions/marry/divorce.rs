use luro_framework::{CommandInteraction, Luro, LuroCommand};
use luro_model::{types::Marriage, COLOUR_DANGER};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::UserMarker, Id};

use super::buttons;

#[derive(CommandModel, CreateCommand)]
#[command(name = "divorce", desc = "It's over.")]
pub struct Divorce {
    /// Who no longer deserves your love?
    pub user: Id<UserMarker>,
    /// Why do they no longer desrve your love?
    reason: String,
}

impl LuroCommand for Divorce {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let proposer = ctx.author.clone();
        let proposee = ctx.fetch_user(self.user).await?;

        ctx.database
            .sqlx
            .marriage_update(Marriage {
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
                        .thumbnail(|t| t.url(proposer.avatar_url()))
                        .create_field("Their Reason", &self.reason, false)
                        .create_field("Approvers", "None!", false)
                        .create_field("Disapprovers", "None!", false)
                })
                .content(format!("Bad news <@{}>...", &self.user))
                .add_components(buttons())
        })
        .await
    }
}
