use anyhow::Context;
use luro_framework::{CommandInteraction, Luro, LuroCommand};
use luro_model::user::marriage::Marriage;
use rand::{seq::SliceRandom, thread_rng};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::UserMarker, Id};

use super::{buttons, MARRIAGE_REASONS};

#[derive(CommandModel, CreateCommand)]
#[command(name = "someone", desc = "Propose to someone! So lucky, aww~")]
pub struct Someone {
    /// Set this if you want to marry someone!
    pub marry: Id<UserMarker>,
    /// The reason you wish to marry them!
    reason: Option<String>,
}

impl LuroCommand for Someone {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let proposer = ctx.author.clone();
        let accent_colour = ctx.accent_colour();

        let reason = self.reason.unwrap_or(
            MARRIAGE_REASONS
                .choose(&mut thread_rng())
                .context("Expected to be able to choose a random reason")?
                .replace("<user>", &format!("<@{}>", &self.marry))
                .replace("<author>", &format!("<@{}>", &proposer.user_id)),
        );

        ctx.database
            .driver
            .marriage_update(Marriage {
                proposer_id: proposer.user_id,
                proposee_id: self.marry,
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
                        .thumbnail(|t| t.url(proposer.avatar_url()))
                        .create_field("Their Reason", &reason, false)
                        .create_field("Approvers", "None!", false)
                        .create_field("Disapprovers", "None!", false)
                })
                .content(format!("<@{}>", &self.marry))
                .add_components(buttons())
        })
        .await
    }
}
