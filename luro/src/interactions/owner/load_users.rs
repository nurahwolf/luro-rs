use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::http::interaction::InteractionResponseType;

use crate::interaction::LuroSlash;
use crate::luro_command::LuroCommand;
use luro_model::database::drivers::LuroDatabaseDriver;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "load_users",
    desc = "For every user in the cache, load their data from the database. This is SLOW!"
)]
pub struct OwnerLoadUsers {}

impl LuroCommand for OwnerLoadUsers {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let response = InteractionResponseType::DeferredChannelMessageWithSource;
        ctx.respond(|r| r.response_type(response)).await?;

        let mut loaded = 0;
        let mut errors = 0;

        for user in ctx.framework.twilight_cache.iter().users() {
            match ctx.framework.database.get_user(&user.id).await {
                Ok(mut user_data) => {
                    user_data.accent_color = user.accent_color;
                    user_data.avatar = user.avatar;
                    user_data.banner = user.banner;
                    user_data.bot = user.bot;
                    user_data.discriminator = user.discriminator().get();
                    user_data.global_name = user.global_name.clone();
                    user_data.email = user.email.clone();
                    user_data.flags = user.flags;
                    user_data.id = user.id.get();
                    user_data.locale = user.locale.clone();
                    user_data.mfa_enabled = user.mfa_enabled.unwrap_or_default();
                    user_data.name = user.name.clone();
                    user_data.premium_type = user.premium_type;
                    user_data.public_flags = user.public_flags;
                    user_data.system = user.system.unwrap_or_default();
                    user_data.verified = user.verified.unwrap_or_default();
                    if ctx.framework.database.save_user(&user.id, &user_data).await.is_err() {
                        errors += 1
                    }

                    loaded += 1;
                }
                Err(_) => errors += 1
            }
        }

        ctx.respond(|r| {
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
