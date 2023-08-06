use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::models::{LuroResponse, UserData};
use crate::LuroContext;

use crate::traits::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "load_users",
    desc = "For every user in the cache, load their data from the database. This is SLOW!"
)]
pub struct OwnerLoadUsers {}

#[async_trait]
impl LuroCommand for OwnerLoadUsers {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        ctx.deferred(&mut slash).await?;

        let mut loaded = 0;
        let mut errors = 0;

        for user in ctx.twilight_cache.iter().users() {
            match UserData::modify_user_settings(ctx, &user.id).await {
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
            slash
                .content(format!(
                    "Loaded {loaded} users! I failed to load a total of `{errors}` users though. Sorry!"
                ))
                .ephemeral();
            ctx.respond(&mut slash).await
        } else {
            slash
                .content(format!("Loaded {loaded} users with no errors! Awesome!"))
                .ephemeral();
            ctx.respond(&mut slash).await
        }
    }
}
