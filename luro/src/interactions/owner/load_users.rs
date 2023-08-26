use crate::luro_command::LuroCommand;
use crate::{interaction::LuroSlash, USERDATA_FILE_PATH};
use luro_model::database::drivers::LuroDatabaseDriver;
use tokio::fs::read_dir;
use tracing::{info, warn};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::Id;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "load_users",
    desc = "For every user in the cache, load their data from the database. This is SLOW!"
)]
pub struct OwnerLoadUsers {
    /// True: Load ALL users that exist in the DB. False: Load only from the cache
    from_db: bool
}

impl LuroCommand for OwnerLoadUsers {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let response = ctx.acknowledge_interaction(true).await?;

        let (loaded, errors) = match self.from_db {
            true => load_disk(&ctx).await?,
            false => load_cache(&ctx).await?
        };

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

async fn load_cache<D: LuroDatabaseDriver>(ctx: &LuroSlash<D>) -> anyhow::Result<(usize, usize)> {
    let mut loaded = 0;
    let mut errors = 0;

    for user in ctx.framework.twilight_cache.iter().users() {
        let mut user_data = match ctx
            .framework
            .database
            .get_user_cached(&user.id, &ctx.framework.twilight_cache)
            .await
        {
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

    Ok((loaded, errors))
}

async fn load_disk<D: LuroDatabaseDriver>(ctx: &LuroSlash<D>) -> anyhow::Result<(usize, usize)> {
    let mut loaded = 0;
    let mut errors = 0;

    let mut paths = read_dir(USERDATA_FILE_PATH).await?;

    match paths.next_entry().await {
        Ok(entry) => match entry {
            Some(entry) => match entry.file_name().into_string() {
                Ok(file) => {
                    info!("Name: {file}");
                    match ctx.framework.database.get_user(&Id::new(file.parse()?)).await {
                        Ok(_) => loaded += 1,
                        Err(_) => errors += 1
                    }
                }
                Err(why) => {
                    warn!(why = ?why, "Failed to load user");
                    errors += 1;
                }
            },
            None => {
                warn!("No data in entry");
                errors += 1;
            }
        },
        Err(why) => {
            warn!(why = ?why, "Failed to load user");
            errors += 1;
        }
    }

    Ok((loaded, errors))
}
