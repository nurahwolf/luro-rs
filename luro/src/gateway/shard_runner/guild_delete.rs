use twilight_gateway::MessageSender;
use twilight_model::gateway::payload::incoming::GuildDelete;

use crate::gateway::GatewayArc;

pub fn guild_delete_handler(_gateway: GatewayArc, _shard: MessageSender, event: GuildDelete) {
    tracing::info!("Left guild '{}'", event.id)
}
