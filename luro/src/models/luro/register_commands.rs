use twilight_model::application::command::Command;

use crate::gateway::GatewayResult;

impl super::Luro {
    /// Register commands to the Discord API.
    pub async fn register_commands(&self, commands: &[Command]) -> GatewayResult {
        let client = self.interaction_client();

        if let Ok(commands) = client.set_global_commands(commands).await?.model().await {
            tracing::info!("Successfully registered {} global commands!", commands.len())
        }

        Ok(())
    }
}
