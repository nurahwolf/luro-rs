use luro_framework::{CommandInteraction, Luro, LuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "flush",
    desc = "Drop the database cache and reinitialise it, useful for if data has changed on the backend"
)]
pub struct Flush {}

impl LuroCommand for Flush {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        ctx.acknowledge_interaction(false).await?;
        let mut users = 0;
        let mut channels = 0;
        let mut guild = 0;
        let mut errors = 0;

        for message in ctx.database.get_messages().await.values() {
            match ctx.fetch_user(&message.author.id, false).await {
                Ok(_) => users += 1,
                Err(_) => errors += 1,
            }

            match ctx.fetch_channel(message.channel_id, false).await {
                Ok(_) => channels += 1,
                Err(_) => errors += 1,
            }

            if let Some(guild_id) = message.guild_id {
                match ctx.get_guild(guild_id, false).await {
                    Ok(_) => guild += 1,
                    Err(_) => errors += 1,
                }
            }
        }

        ctx.respond(|r| {
            r.embed(|embed| {
                embed
                    .title("Database flushed!")
                    .description(format!("- Updated `{users}` users\n
                    - Updated `{channels}` channels\n
                    - Updated `{guild}` guilds\n
                    - Had `{errors}` errors!"))
                    .colour(ctx.accent_colour())
            })
            .deferred()
        })
        .await
    }
}
