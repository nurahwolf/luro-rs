use crate::luro_command::LuroCommand;
use crate::{interaction::LuroSlash, USERDATA_FILE_PATH};
use luro_model::database_driver::LuroDatabaseDriver;
use tokio::fs::read_dir;
use tracing::{debug, warn};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::Id;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "load_users",
    desc = "For every user in the cache, load their data from the database. This is SLOW!"
)]
pub struct OwnerLoadUsers {
    /// True: Load ALL users that exist in the DB. False: Load only from the cache
    from_db: bool,
}

impl LuroCommand for OwnerLoadUsers {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let response = ctx.acknowledge_interaction(true).await?;

        let (loaded, errors) = match self.from_db {
            true => load_disk(&ctx).await?,
            false => load_cache(&ctx).await?,
        };

        ctx.respond(|r| {
            *r = response;
            match errors != 0 {
                true => r.content(format!(
                    "Loaded {loaded} users! I failed to load a total of `{errors}` users though. Sorry!"
                )),
                false => r.content(format!("Loaded {loaded} users with no errors! Awesome!")),
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
        let mut user_data = match ctx.framework.database.get_user(&user.id).await {
            Ok(data) => data,
            Err(why) => {
                warn!(why = ?why, "Failed to fetch {:#?} user for the following reason:", user.id);
                errors += 1;
                continue;
            }
        };

        user_data.update_user(&user);
        if ctx.framework.database.modify_user(&user.id, &user_data).await.is_err() {
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

    while let Some(entry) = paths.next_entry().await? {
        debug!("{:?}", entry.file_name());

        if let Some(user_id) = entry.file_name().to_str() {
            let user_id = &Id::new(user_id.parse()?);

            if let Ok(user_data) = ctx.framework.database.user_data.read() {
                // User is already present in the cache
                if user_data.get(user_id).is_some() {
                    loaded += 1;
                    continue;
                }
            }

            match ctx.framework.database.get_user(user_id).await {
                Ok(_) => loaded += 1,
                Err(_) => errors += 1,
            }
        }
    }

    Ok((loaded, errors))
}
