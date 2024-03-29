use std::convert::TryFrom;

use luro_framework::{CommandInteraction, Luro, LuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "clear_warnings", desc = "Clears a user's warning by ID.")]
pub struct Warnings {
    /// The user to clear
    pub user: ResolvedUser,
    /// The warning ID to remove. Removes all if not set.
    pub id: Option<i64>,
    /// Also remove ALL recorded punishments.
    pub clear_punishments: Option<bool>,
}

impl LuroCommand for Warnings {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let mut user_data = ctx.fetch_user(&self.user.resolved.id).await?;
        if user_data.warnings.is_empty() {
            return ctx
                .respond(|r| r.content("User has no warnings you stupid idiot!").ephemeral())
                .await;
        }
        if let Some(index) = self.id {
            let index: usize = match usize::try_from(index) {
                Ok(index) => match index.checked_sub(1) {
                    Some(index) => index,
                    None => {
                        return ctx
                            .respond(|r| {
                                r.content("This function automatically reduces the ID by 1. You just had the buffer underflow")
                                    .ephemeral()
                            })
                            .await
                    }
                },
                Err(why) => {
                    return ctx
                        .respond(|r| r.content(format!("Failed to convert `i64` to `usize`\n```{}```!", why)).ephemeral())
                        .await
                }
            };

            if index > user_data.warnings.len() || user_data.warnings.is_empty() {
                return ctx
                    .respond(|r| {
                        r.content(format!(
                            "The vector has {} elements. You are trying to remove a number greater than that.",
                            user_data.warnings.len()
                        ))
                        .ephemeral()
                    })
                    .await;
            }
            user_data.warnings.remove(index);
        } else {
            // Drain all warnings
            user_data.warnings.drain(..);
        }

        if let Some(clear_punishments) = self.clear_punishments && clear_punishments {
            user_data.moderation_actions.drain(..);
        }

        ctx.database.update_user(user_data).await?;
        ctx.respond(|r| r.content("Warnings removed!").ephemeral()).await
    }
}
