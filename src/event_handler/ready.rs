use std::sync::Arc;

use anyhow::Error;
use tracing::{info, warn};
use twilight_model::gateway::payload::incoming::Ready;

use crate::{commands, luro::Luro};

pub async fn ready_listener(luro: Arc<Luro>, ready: Box<Ready>) -> Result<(), Error> {
    info!("Luro is now ready!");
    info!("Username: {} ({})", ready.user.name, ready.user.id);

    let interaction = luro.twilight_client.interaction(luro.application.id);

    match commands::register_global_commands(&interaction, luro.data.global_commands.clone()).await
    {
        Ok(commands) => info!("Registered {} global commands", commands.len()),
        Err(why) => warn!("Failed to register global commands - {why}"),
    };

    Ok(())
}

// async fn save_guild_accent_colour(luro: &Arc<LuroContext>, ready: Box<Ready>) -> Result<()> {
//     let mut guilds = luro.guild_settings.write().await;

//     debug!("Attempting to add guilds to guild_settings.toml");
//     for guild in ready.guilds {
//         match guilds.guilds.entry(guild.id.to_string()) {
//             Entry::Occupied(occupied) => {
//                 occupied.into_mut().accent_colour = 0xdabeef;
//             }
//             Entry::Vacant(vacant) => {
//                 let guild_settings = LuroGuildSettings {
//                     accent_colour: 0xdabeef,
//                     accent_colour_custom: None,
//                     discord_events_log_channel: None,
//                     moderator_actions_log_channel: None,
//                 };

//                 vacant.insert(guild_settings);
//             }
//         };
//     }

//     if let Err(why) = guilds.write().await {
//         warn!("Failed to save guild_settings to the toml file - {why}");
//     };

//     Ok(())
// }

// async fn save_guild_accent_colour(luro: &Arc<Luro>) -> Result<()> {
//     let guild_settings = match luro.guild_settings.read() {
//         Ok(ok) => ok,
//         Err(why) => {
//             warn!("Guild Settings is poisoned, so no guild has been updated: {why}");
//             return Ok(());
//         }
//     };

// }
