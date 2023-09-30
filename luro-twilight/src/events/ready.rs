use luro_framework::{Context, Luro};
use luro_model::{BOT_OWNERS, PRIMARY_BOT_OWNER};
use tracing::{info, warn};
use twilight_model::gateway::{
    payload::{incoming::Ready, outgoing::UpdatePresence},
    presence::{ActivityType, MinimalActivity, Status},
};

use crate::commands::default_commands;

pub async fn ready_listener(framework: Context, event: Box<Ready>) -> anyhow::Result<()> {
    info!("Luro is now ready!");
    info!("==================");
    info!("Username:       {}", event.user.name);
    info!("ID:             {}", event.user.id);
    info!("Guilds:         {}", event.guilds.len());
    info!("API Version:    {}", event.version);
    if let Some(latency) = framework.latency.average() {
        info!("Latency:        {} ms", latency.as_millis());
    }

    let mut presence_string = format!("/about | on {} guilds", event.guilds.len());

    if let Some(shard_id) = event.shard {
        info!("Shard:          {}", shard_id.number());
        info!("Total Shards:   {}", shard_id.total());
        presence_string.push_str(format!(" | shard {}", shard_id.number()).as_str());

        framework.shard.command(&UpdatePresence::new(
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

    let application = framework.twilight_client.current_user_application().await?.model().await?;

    if let Some(owner) = &application.owner {
        info!("App Owner:      {}", owner.name);
    }

    let mut owners = String::new();
    let mut administrators = String::new();
    let mut staff = framework.database.get_staff().await;

    if staff.is_empty() {
        info!("-- Registering Staff in DB --");

        // Register staff
        for staff in BOT_OWNERS {
            framework
                .database
                .register_staff(framework.twilight_client.user(staff).await?.model().await?)
                .await?;
        }

        // Register primary owner
        framework
            .database
            .register_owner(framework.twilight_client.user(PRIMARY_BOT_OWNER).await?.model().await?)
            .await?;

        staff = framework.database.get_staff().await;
    }

    for staff in staff.values() {
        match staff.user_permissions {
            luro_model::user::LuroUserPermissions::Owner => match owners.is_empty() {
                true => owners.push_str(&staff.name),
                false => owners.push_str(format!(", {}", staff.name).as_str()),
            },
            luro_model::user::LuroUserPermissions::Administrator => match administrators.is_empty() {
                true => administrators.push_str(&staff.name),
                false => administrators.push_str(format!(", {}", staff.name).as_str()),
            },
            _ => warn!("User {:#?} is tagged as a regular user in the database!", staff),
        }
    }

    info!("Owners:         {owners}");
    info!("Administrators: {administrators}");

    let commands = default_commands();
    framework.register_commands(&commands).await?;

    Ok(())
}
