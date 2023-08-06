use std::convert::TryFrom;
use std::path::Path;

use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

use crate::models::{LuroResponse, UserData};
use crate::{LuroContext, USERDATA_FILE_PATH};

use crate::traits::luro_command::LuroCommand;
use crate::traits::toml::LuroTOML;

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

#[async_trait]
impl LuroCommand for OwnerClearWarning {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, self.user.resolved.id);
        let path = Path::new(&path);
        let mut user_data = UserData::get_user_settings(ctx, &self.user.resolved.id).await?;
        if user_data.warnings.is_empty() {
            {
                slash.content("User has no warnings you stupid idiot!");
                return ctx.respond(&mut slash).await;
            }
        }
        if let Some(index) = self.id {
            let index: usize = match usize::try_from(index) {
                Ok(index) => match index.checked_sub(1) {
                    Some(index) => index,
                    None => {
                        slash.content("This function automatically reduces the ID by 1. You just had the buffer underflow");
                        return ctx.respond(&mut slash).await;
                    }
                },
                Err(_) => {
                    slash.content("Failed to convert ID to usize");
                    return ctx.respond(&mut slash).await;
                }
            };

            if index > user_data.warnings.len() || user_data.warnings.is_empty() {
                slash.content(format!(
                    "The vector has {} elements. You are trying to remove a number greater than that.",
                    user_data.warnings.len()
                ));
                return ctx.respond(&mut slash).await;
            }
            user_data.warnings.remove(index);
        } else {
            // Drain all warnings
            user_data.warnings.drain(..);
        }

        if let Some(clear_punishments) = self.clear_punishments && clear_punishments {
            user_data.moderation_actions.drain(..);
        }

        ctx.data_user.insert(self.user.resolved.id, user_data.clone());
        user_data.write(path).await?;

        slash.content("Warning removed!").ephemeral();
        ctx.respond(&mut slash).await
    }
}
