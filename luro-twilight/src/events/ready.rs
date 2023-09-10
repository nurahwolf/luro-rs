use luro_framework::{Context, Framework};
use luro_model::database_driver::LuroDatabaseDriver;
use tracing::info;
use twilight_model::gateway::{
    payload::{incoming::Ready, outgoing::UpdatePresence},
    presence::{ActivityType, MinimalActivity, Status},
};

pub async fn ready_listener<D: LuroDatabaseDriver>(
    framework: Framework<D>,
    ctx: Context,
    event: Box<Ready>,
) -> anyhow::Result<()> {
    info!("Luro is now ready!");
    info!("==================");
    info!("Username:      {}", event.user.name);
    info!("ID:            {}", event.user.id);
    info!("Guilds:        {}", event.guilds.len());
    info!("API Version:   {}", event.version);
    if let Some(latency) = ctx.latency.average() {
        info!("Latency:       {} ms", latency.as_millis());
    }

    let mut presence_string = format!("/about | on {} guilds", event.guilds.len());

    if let Some(shard_id) = event.shard {
        info!("Shard:         {}", shard_id.number());
        info!("Total Shards:  {}", shard_id.total());
        presence_string.push_str(format!(" | shard {}", shard_id.number()).as_str());

        ctx.shard.command(&UpdatePresence::new(
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
        info!("Primary Owner: {}", owner.name);
    }
    let mut owners = String::new();
    let staff = framework.database.get_staff().await?;

    for staff in staff.values() {
        if owners.is_empty() {
            owners.push_str(&staff.name)
        } else {
            owners.push_str(format!(", {}", staff.name).as_str())
        }
    }
    info!("Owners:        {owners}");

    framework.register_commands(None).await?;

    Ok(())
}
