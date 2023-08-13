use std::convert::TryFrom;



use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

use crate::slash::Slash;
use crate::traits::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "clear_warnings", desc = "Clears a user's warning by ID.")]
pub struct OwnerClearWarning {
    /// The user to clear
    pub user: ResolvedUser,
    /// The warning ID to remove. Removes all if not set.
    pub id: Option<i64>,
    /// Also remove ALL recorded punishments.
    pub clear_punishments: Option<bool>
}


impl LuroCommand for OwnerClearWarning {
    async fn run_command(self, mut ctx: Slash) -> anyhow::Result<()> {
        let mut user_data = ctx.framework.database.get_user(&self.user.resolved.id).await?;
        if user_data.warnings.is_empty() {
            return ctx.content("User has no warnings you stupid idiot!").respond().await;
        }
        if let Some(index) = self.id {
            let index: usize = match usize::try_from(index) {
                Ok(index) => match index.checked_sub(1) {
                    Some(index) => index,
                    None => {
                        return ctx
                            .clone()
                            .content("This function automatically reduces the ID by 1. You just had the buffer underflow")
                            .respond()
                            .await
                    }
                },
                Err(_) => return ctx.content("Failed to convert ID to usize").respond().await
            };

            if index > user_data.warnings.len() || user_data.warnings.is_empty() {
                return ctx
                    .clone()
                    .content(format!(
                        "The vector has {} elements. You are trying to remove a number greater than that.",
                        user_data.warnings.len()
                    ))
                    .respond()
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

        ctx.framework.database.modify_user(&self.user.resolved.id, &user_data).await?;
        ctx.content("Warning removed!").ephemeral().respond().await
    }
}
