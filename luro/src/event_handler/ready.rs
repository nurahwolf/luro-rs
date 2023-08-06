use tracing::{debug, info};
use twilight_gateway::MessageSender;
use twilight_model::gateway::{
    payload::{incoming::Ready, outgoing::UpdatePresence},
    presence::{ActivityType, MinimalActivity, Status}
};

use crate::{models::GuildSetting, LuroFramework};

impl LuroFramework {
    pub async fn ready_listener(&self, ready: Box<Ready>, shard: MessageSender) -> anyhow::Result<()> {
        let mut presence_string = "/about".to_owned();
        info!("Luro is now ready!");
        info!("==================");
        info!("Username:      {}", ready.user.name);
        info!("ID:            {}", ready.user.id);
        info!("Guilds:        {}", ready.guilds.len());
        info!("API Version:   {}", ready.version);

        presence_string.push_str(format!(" | on {} guilds", ready.guilds.len()).as_str());

        if let Some(shard_id) = ready.shard {
            info!("Shard:         {}", shard_id.number());
            info!("Total Shards:  {}", shard_id.total());
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
            info!("Primary Owner: {}", owner.name);
        }
        let mut owners = String::new();
        for owner in &self.global_data.read().owners {
            if owners.is_empty() {
                owners.push_str(&owner.name)
            } else {
                owners.push_str(format!(", {}", owner.name).as_str())
            }
        }
        info!("Owners:        {owners}");

        debug!("Attempting to register guild settings");
        self.register_commands(application.id).await?;

        for guild in ready.guilds {
            GuildSetting::get_guild_settings(self, &guild.id).await?;
        }

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
