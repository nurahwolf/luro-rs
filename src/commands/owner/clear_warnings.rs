use std::convert::TryFrom;
use std::path::Path;

use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

use crate::models::{LuroSlash, UserData};
use crate::USERDATA_FILE_PATH;

use crate::traits::luro_command::LuroCommand;
use crate::traits::toml::LuroTOML;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "clear_warnings", desc = "Clears a user's warning by ID.")]
pub struct OwnerClearWarning {
    /// The user to clear
    pub user: ResolvedUser,
    /// The warning ID to remove. Removes all if not set.
    pub id: Option<i64>
}

#[async_trait]
impl LuroCommand for OwnerClearWarning {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, self.user.resolved.id);
        let path = Path::new(&path);
        let mut user_data = UserData::get_user_settings(&ctx.luro, &self.user.resolved.id).await?;
        match user_data.warnings.as_mut() {
            Some(warnings) => {
                if let Some(index) = self.id {
                    let index: usize =
                        match usize::try_from(index) {
                            Ok(index) => match index.checked_sub(1) {
                                Some(index) => index,
                                None => return ctx
                                    .content(
                                        "This function automatically reduces the ID by 1. You just had the buffer underflow"
                                    )
                                    .respond()
                                    .await
                            },
                            Err(_) => return ctx.content("Failed to convert ID to usize").respond().await
                        };

                    if index > warnings.len() || warnings.is_empty() {
                        return ctx
                            .content(format!(
                                "The vector has {} elements. You are trying to remove a number greater than that.",
                                warnings.len()
                            ))
                            .respond()
                            .await;
                    }
                    warnings.remove(index);
                } else {
                    // Drain all warnings
                    warnings.drain(..);
                }
            }
            None => return ctx.content("User has no warnings you stupid idiot!").respond().await
        }

        ctx.luro.user_data.insert(self.user.resolved.id, user_data.clone());
        user_data.write(path).await?;

        ctx.content("Warning removed!").ephemeral().respond().await
    }
}
