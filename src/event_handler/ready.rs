use tracing::{debug, info};
use twilight_gateway::MessageSender;
use twilight_model::gateway::{
    payload::{incoming::Ready, outgoing::UpdatePresence},
    presence::{ActivityType, MinimalActivity, Status}
};

use crate::LuroFramework;

impl LuroFramework {
    pub async fn ready_listener(&self, ready: Box<Ready>, shard: MessageSender) -> anyhow::Result<()> {
        let mut presence_string = "/about".to_owned();
        info!("Luro is now ready!");
        info!("==================");
        info!("Username:     {}", ready.user.name);
        info!("ID:           {}", ready.user.id);
        info!("Guilds:       {}", ready.guilds.len());
        info!("API Version:  {}", ready.version);

        presence_string.push_str(format!(" | on {} guilds", ready.guilds.len()).as_str());

        if let Some(shard_id) = ready.shard {
            info!("Shard:        {}", shard_id.number());
            info!("Total Shards: {}", shard_id.total());
            presence_string.push_str(format!(" | shard {}", shard_id.number()).as_str());

            shard.command(&UpdatePresence::new(
                vec![MinimalActivity {
                    kind: ActivityType::Playing,
                    name: presence_string,
                    url: None
                }
                .into()],
                false,
                None,
                Status::Online
            )?)?;
        };

        let application = self.twilight_client.current_user_application().await?.model().await?;

        if let Some(owner) = &application.owner {
            info!("Owner:        {}", owner.name);
        }

        debug!("Attempting to register guild settings");
        self.register_commands(application.id).await?;

        // match luro.application.try_read() {
        //     Ok(application_data) => {
        //         let interaction_client = luro.twilight_client.interaction(application_data.id);

        //         match luro.global_commands.try_read() {
        //             Ok(commands) => {
        //                 match commands::register_global_commands(&interaction_client, commands.clone())
        //                     .await
        //                 {
        //                     Ok(commands) => info!("Registered {} global commands", commands.len()),
        //                     Err(why) => warn!("Failed to register global commands - {why}"),
        //                 };
        //             }
        //             Err(why) => warn!(?why, "Failed to get the list of global commands"),
        //         };
        //     }
        //     Err(why) => {
        //         warn!("Failed to read application data, no commands were registered: {why}")
        //     }
        // }

        Ok(())
    }
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
