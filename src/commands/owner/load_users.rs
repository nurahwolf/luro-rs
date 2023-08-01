use async_trait::async_trait;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::models::{LuroSlash, UserData};

use crate::traits::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "load_users",
    desc = "For every user in the cache, load their data from the database. This is SLOW!"
)]
pub struct OwnerLoadUsers {}

#[async_trait]
impl LuroCommand for OwnerLoadUsers {
    async fn run_command(self, mut ctx: LuroSlash) -> anyhow::Result<()> {
        ctx.deferred().await?;

        let mut loaded = 0;
        let mut errors = 0;

        for user in ctx.luro.twilight_cache.iter().users() {
            match UserData::get_user_settings(&ctx.luro, &user.id).await {
                Ok(_) => loaded += 1,
                Err(_) => errors += 1
            }
        }

        if errors != 0 {
            ctx.content(format!(
                "Loaded {loaded} users! I failed to load a total of `{errors}` users though. Sorry!"
            ))
            .ephemeral()
            .respond()
            .await
        } else {
            ctx.content(format!("Loaded {loaded} users with no errors! Awesome!"))
                .ephemeral()
                .respond()
                .await
        }
    }
}
