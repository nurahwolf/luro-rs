use std::{collections::hash_map::Entry, sync::Arc};

use anyhow::Result;
use tracing::{debug, info, warn};
use twilight_model::gateway::payload::incoming::Ready;

use crate::{config::LuroGuildSettings, LuroContext, Luro};

pub async fn ready_handler(luro: Arc<Luro>, ready: Box<Ready>) -> Result<()> {
    info!("Luro is now ready!");
    info!("Username: {} ({})", ready.user.name, ready.user.id);

    if let Err(why) = luro.register_global_commands().await {
        warn!("Failed to register global commands - {why}")
    };

    if let Err(why) = save_guild_accent_colour(&luro, ready).await {
        warn!("Failed to save guild accent colours - {why}")
    };

    Ok(())
}

async fn save_guild_accent_colour(luro: &Arc<LuroContext>, ready: Box<Ready>) -> Result<()> {
    let mut guilds = luro.guild_settings.write().await;

    debug!("Attempting to add guilds to guild_settings.toml");
    for guild in ready.guilds {
        match guilds.guilds.entry(guild.id.to_string()) {
            Entry::Occupied(occupied) => {
                occupied.into_mut().accent_colour = 0xdabeef;
            }
            Entry::Vacant(vacant) => {
                let guild_settings = LuroGuildSettings {
                    accent_colour: 0xdabeef,
                    accent_colour_custom: None,
                    discord_events_log_channel: None,
                    moderator_actions_log_channel: None,
                };

                vacant.insert(guild_settings);
            }
        };
    }

    if let Err(why) = guilds.write().await {
        warn!("Failed to save guild_settings to the toml file - {why}");
    };

    Ok(())
}

// async fn save_guild_accent_colour(luro: &Arc<Luro>) -> Result<()> {
//     let guild_settings = match luro.guild_settings.read() {
//         Ok(ok) => ok,
//         Err(why) => {
//             warn!("Guild Settings is poisoned, so no guild has been updated: {why}");
//             return Ok(());
//         }
//     };

// }
