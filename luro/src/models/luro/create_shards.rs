use std::sync::Arc;

use twilight_gateway::Shard;

use crate::{database::Database, models::Config};

use super::GatewayError;

impl super::Luro {
    /// Start up the gateway. This then returns the instanced shards for handling elsewhere
    ///
    /// NOTE: This attempts to load the environmental variable `DISCORD_TOKEN`.
    /// Then it will attempt to find a configuration file (or create one) with the bot name.
    ///
    /// If either of these fail, the gateway will panic and exit.
    pub async fn create_shards(
        intents: twilight_gateway::Intents,
    ) -> Result<(Arc<Self>, impl ExactSizeIterator<Item = Shard>), GatewayError> {
        tracing::info!("GATEWAY: Attempting to start up!");

        // Attempts to get the `DISCORD_TOKEN` environmentable variable, else panic
        let Ok(discord_token) = std::env::var("DISCORD_TOKEN") else {
            return Err(GatewayError::NoToken);
        };

        // Setup bot essentials
        let twilight = Arc::new(twilight_http::Client::new(discord_token.clone()));
        let current_user = twilight.current_user().await?.model().await?;
        let current_user_name = current_user.name.to_lowercase();
        let gateway_config = Config::fetch(&format!("./config/{current_user_name}.toml")).await?;
        let http_client = reqwest::ClientBuilder::new().build()?;
        let database = Database::new(&gateway_config, twilight.clone()).await?;
        let application = twilight.current_user_application().await?.model().await?;

        // Create each shard in a set, based on Discord's recommendations
        let shards =
            twilight_gateway::create_recommended(&twilight, twilight_gateway::Config::new(discord_token, intents), |_, c| c.build())
                .await?;

        tracing::info!("GATEWAY: Finished setting up! Now kicking off the shards...");

        Ok((
            Self {
                application,
                config: gateway_config,
                current_user: current_user.into(),
                database,
                http_client,
                twilight_client: twilight,
                shard: None,
            }
            .into(),
            shards,
        ))
    }
}
