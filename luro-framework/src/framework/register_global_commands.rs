use anyhow::anyhow;
use luro_model::database::drivers::LuroDatabaseDriver;
use tracing::info;
use twilight_model::{
    application::command::Command,
    id::{marker::ApplicationMarker, Id}
};

use crate::Framework;

impl<D: LuroDatabaseDriver> Framework<D> {
    /// Register the commands that are within the framework's global command cache
    pub async fn register_global_commands(&self, application_id: Id<ApplicationMarker>) -> anyhow::Result<()> {
        let client = self.interaction_client(application_id);
        let commands = match self.global_commands.lock() {
            Ok(mutex) => mutex.clone().into_iter().map(|x| x.1).collect::<Vec<Command>>(),
            Err(why) => return Err(anyhow!("Failed to lock: {why}"))
        };

        client.set_global_commands(&commands).await?;
        info!("Successfully registered {} global commands!", commands.len());

        Ok(())
    }
}
