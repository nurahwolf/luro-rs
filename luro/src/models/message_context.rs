pub struct MessageContext {
    pub gateway: crate::gateway::GatewayArc,
    pub shard: twilight_gateway::MessageSender,
    pub ctx: Box<twilight_model::gateway::payload::incoming::MessageCreate>,
}
