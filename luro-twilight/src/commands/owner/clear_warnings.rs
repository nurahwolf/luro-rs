use std::convert::TryFrom;

use async_trait::async_trait;
use luro_framework::{command::LuroCommandTrait, Framework, InteractionCommand, LuroInteraction};
use luro_model::database_driver::LuroDatabaseDriver;
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

#[async_trait]
impl LuroCommandTrait for Warnings {
    async fn handle_interaction(
        ctx: Framework,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        let mut user_data = ctx.database.get_user(&data.user.resolved.id).await?;
        if user_data.warnings.is_empty() {
            return interaction
                .respond(&ctx, |r| r.content("User has no warnings you stupid idiot!").ephemeral())
                .await;
        }
        if let Some(index) = data.id {
            let index: usize = match usize::try_from(index) {
                Ok(index) => match index.checked_sub(1) {
                    Some(index) => index,
                    None => {
                        return interaction
                            .respond(&ctx, |r| {
                                r.content("This function automatically reduces the ID by 1. You just had the buffer underflow")
                                    .ephemeral()
                            })
                            .await
                    }
                },
                Err(why) => {
                    return interaction
                        .respond(&ctx, |r| {
                            r.content(format!("Failed to convert `i64` to `usize`\n```{}```!", why.to_string()))
                                .ephemeral()
                        })
                        .await
                }
            };

            if index > user_data.warnings.len() || user_data.warnings.is_empty() {
                return interaction
                    .respond(&ctx, |r| {
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

        if let Some(clear_punishments) = data.clear_punishments && clear_punishments {
            user_data.moderation_actions.drain(..);
        }

        ctx.database.modify_user(&data.user.resolved.id, &user_data).await?;
        interaction
            .respond(&ctx, |r| r.content("Warnings removed!").ephemeral())
            .await
    }
}
