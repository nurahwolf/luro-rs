use twilight_gateway::{Event, Latency, MessageSender};
use twilight_http::client::InteractionClient;

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
            lavalink: framework.lavalink,
            shard,
            tracing_subscriber: framework.tracing_subscriber,
            twilight_client: framework.twilight_client,
        }
    }
}

impl Luro for Context {
    /// Gets the [interaction client](InteractionClient) using this framework's
    /// [http client](Client) and [application id](ApplicationMarker)
    fn interaction_client(&self) -> InteractionClient {
        self.twilight_client.interaction(self.database.application.read().unwrap().id)
    }
}