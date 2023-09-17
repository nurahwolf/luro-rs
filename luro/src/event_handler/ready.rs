use luro_model::database_driver::LuroDatabaseDriver;
use tracing::{debug, info};
use twilight_gateway::MessageSender;
use twilight_model::gateway::{
    payload::{incoming::Ready, outgoing::UpdatePresence},
    presence::{ActivityType, MinimalActivity, Status},
};

use crate::framework::Framework;

impl<D: LuroDatabaseDriver> Framework {
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
                    url: None,
                }
                .into()],
                false,
                None,
                Status::Online,
            )?)?;
        };

        let application = self.twilight_client.current_user_application().await?.model().await?;

        if let Some(owner) = &application.owner {
            info!("Primary Owner: {}", owner.name);
        }
        let mut owners = String::new();
        let staff = self.database.get_staff().await?;

        for staff in staff.values() {
            if owners.is_empty() {
                owners.push_str(&staff.name)
            } else {
                owners.push_str(format!(", {}", staff.name).as_str())
            }
        }
        info!("Owners:        {owners}");

        debug!("Attempting to register guild settings");
        self.register_commands(application.id).await?;

        Ok(())
    }
}
