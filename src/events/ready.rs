use tracing::info;
use twilight_model::gateway::payload::incoming::Ready;

use crate::State;

pub async fn ready_handler(_state: State, ready: Box<Ready>) -> anyhow::Result<()> {
    info!("Luro is now ready!");
    info!("Username: {} ({})", ready.user.name, ready.user.id);

    if let Err(why) = luro.register_global_commands().await {
        warn!("Failed to register global commands - {why}")
    };

    // if let Err(why) = save_guild_accent_colour(&luro, ready).await {
    //     warn!("Failed to save guild accent colours - {why}")
    // };

    Ok(())
}
