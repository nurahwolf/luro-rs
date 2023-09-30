use twilight_gateway::{Event, Latency, MessageSender};

use crate::{Context, Framework, Luro};

impl Context {
    pub fn new(framework: Framework, event: Event, latency: Latency, shard: MessageSender) -> Self {
        Self {
            cache: framework.cache,
            database: framework.database,
            event,
            global_commands: framework.global_commands,
            guild_commands: framework.guild_commands,
            http_client: framework.http_client,
            latency,
            #[cfg(feature = "lavalink")]
            lavalink: framework.lavalink,
            shard,
            tracing_subscriber: framework.tracing_subscriber,
            twilight_client: framework.twilight_client,
        }
    }
}

impl Luro for Context {
    async fn interaction_client(&self) -> anyhow::Result<twilight_http::client::InteractionClient> {
        Ok(self.twilight_client.interaction(self.application().await?.id))
    }

    fn database(&self) -> std::sync::Arc<luro_database::LuroDatabase> {
        self.database.clone()
    }

    fn twilight_client(&self) -> std::sync::Arc<twilight_http::Client> {
        self.twilight_client.clone()
    }
}
