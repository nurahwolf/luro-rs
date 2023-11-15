use std::collections::{hash_map::Entry, HashMap};

use luro_framework::{CommandInteraction, Luro, LuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "flush",
    desc = "Drop the database cache and reinitialise it, useful for if data has changed on the backend"
)]
pub struct Flush {}

impl LuroCommand for Flush {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        ctx.acknowledge_interaction(false).await?;
        let mut users = HashMap::new();
        let mut channels = HashMap::new();
        let mut guilds = HashMap::new();
        let mut errors = 0;

        for message in ctx.database.driver.get_messages().await.values() {
            if let Entry::Vacant(entry) = users.entry(message.author.user_id) {
                entry.insert(());
                if ctx.fetch_user_only(message.author.user_id).await.is_err() {
                    errors += 1
                }
            }

            if let Entry::Vacant(entry) = channels.entry(message.channel_id) {
                entry.insert(());
                if ctx.fetch_channel(message.channel_id).await.is_err() {
                    errors += 1
                }
            }

            if let Some(guild_id) = message.guild_id {
                if let Entry::Vacant(entry) = guilds.entry(guild_id) {
                    entry.insert(());
                    if ctx.get_guild(guild_id).await.is_err() {
                        errors += 1
                    }
                }
            }
        }

        ctx.respond(|r| {
            r.embed(|embed| {
                embed
                    .title("Database flushed!")
                    .description(format!(
                        "- Updated `{}` users\n- Updated `{}` channels\n- Updated `{}` guilds\n- Had `{errors}` errors!",
                        users.len(),
                        channels.len(),
                        guilds.len()
                    ))
                    .colour(ctx.accent_colour())
            })
            .deferred()
        })
        .await
    }
}
