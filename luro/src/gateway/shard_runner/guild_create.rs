use twilight_gateway::MessageSender;
use twilight_model::gateway::payload::incoming::GuildCreate;

use crate::gateway::GatewayArc;

pub fn guild_create_handler(_gateway: GatewayArc, _shard: MessageSender, event: Box<GuildCreate>) {
    tracing::info!("Joined guild {} ({})", event.name, event.id)
}
