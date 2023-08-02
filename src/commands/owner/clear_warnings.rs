use std::convert::TryFrom;
use std::path::Path;

use anyhow::Context;
use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

use crate::models::{LuroSlash, UserData};
use crate::USERDATA_FILE_PATH;

use crate::traits::luro_command::LuroCommand;
use crate::traits::toml::LuroTOML;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "clear_warning", desc = "Clears a user's warning by ID.")]
pub struct OwnerClearWarning {
    /// The user to clear
    pub user: ResolvedUser,
    /// The warning ID to remove
    pub id: i64
}

#[async_trait]
impl LuroCommand for OwnerClearWarning {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, self.id);
        let path = Path::new(&path);
        let mut user_data = UserData::get_user_settings(&ctx.luro, &self.user.resolved.id).await?;
        match user_data.warnings {
            Some(ref mut warnings) => {
                let index: usize = usize::try_from(self.id).context("Attempted to convert i64 to usize")? - 1;
                if index > warnings.len() {
                    return ctx
                        .content(format!(
                            "The vector has {index} elements. You are trying to remove a number greater than that."
                        ))
                        .respond()
                        .await;
                }

                warnings.remove(index);
            }
            None => return ctx.content("User has no warnings you stupid idiot!").respond().await
        }

        ctx.luro.user_data.insert(self.user.resolved.id, user_data.clone());
        user_data.write(path).await?;

        ctx.content("Warning removed!").respond().await
    }
}
