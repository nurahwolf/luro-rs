use async_trait::async_trait;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::models::{LuroSlash, UserData};

use crate::traits::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "config",
    desc = "For every user in the cache, load their data from the database. This is SLOW!"
)]
pub struct ConfigCommand {}

#[async_trait]
impl LuroCommand for ConfigCommand {
    async fn run_command(self, mut ctx: LuroSlash) -> anyhow::Result<()> {
        ctx.deferred().await?;

        let mut loaded = 0;
        let mut errors = 0;

        for user in ctx.luro.twilight_cache.iter().users() {
            match UserData::modify_user_settings(&ctx.luro, &user.id).await {
                Ok(mut user_data) => {
                    user_data.accent_color = user.accent_color;
                    user_data.avatar = user.avatar;
                    user_data.banner = user.banner;
                    user_data.bot = user.bot;
                    user_data.discriminator = Some(user.discriminator().get());
                    user_data.email = user.email.clone();
                    user_data.flags = user.flags;
                    user_data.id = Some(user.id);
                    user_data.locale = user.locale.clone();
                    user_data.mfa_enabled = user.mfa_enabled;
                    user_data.name = Some(user.name.clone());
                    user_data.premium_type = user.premium_type;
                    user_data.public_flags = user.public_flags;
                    user_data.system = user.system;
                    user_data.verified = user.verified;
                    if user_data.write_user_data(&user.id).await.is_err() {
                        errors += 1
                    }

                    loaded += 1;
                }
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
