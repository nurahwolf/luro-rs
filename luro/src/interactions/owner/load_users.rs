use crate::interaction::LuroSlash;
use crate::luro_command::LuroCommand;
use luro_model::database::drivers::LuroDatabaseDriver;
use tracing::warn;
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "load_users",
    desc = "For every user in the cache, load their data from the database. This is SLOW!"
)]
pub struct OwnerLoadUsers {}

impl LuroCommand for OwnerLoadUsers {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let response = ctx.acknowledge_interaction(true).await?;

        let mut loaded = 0;
        let mut errors = 0;

        for user in ctx.framework.twilight_cache.iter().users() {
            let mut user_data = match ctx.framework.database.get_user_cached(&user.id, &ctx.framework.twilight_cache).await {
                Ok(data) => data,
                Err(why) => {
                    warn!(why = ?why, "Failed to fetch {:#?} user for the following reason:", user.id);
                    errors += 1;
                    continue;
                }
            };

            user_data.update_user(&user);
            if ctx.framework.database.save_user(&user.id, &user_data).await.is_err() {
                errors += 1
            }

            loaded += 1
        }

        ctx.respond(|r| {
            *r = response;
            match errors != 0 {
                true => r.content(format!(
                    "Loaded {loaded} users! I failed to load a total of `{errors}` users though. Sorry!"
                )),
                false => r.content(format!("Loaded {loaded} users with no errors! Awesome!"))
            };
            r.ephemeral()
        })
        .await
    }
}
