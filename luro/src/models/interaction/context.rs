use twilight_gateway::Latency;

use crate::gateway::GatewayArc;

mod accent_colour;
mod acknowledge_interaction;
mod author;
mod author_id;
mod author_or_user;
mod bot;
mod channel;
mod command_name;
mod component_message;
mod database;
mod fetch_member;
mod fetch_user;
mod guild;
mod interaction_client;
mod parse_field;
mod respond;
mod response_send;
mod response_update;
mod standard_response;

pub struct InteractionContext {
    pub gateway: GatewayArc,
    pub shard: twilight_gateway::MessageSender,
    pub latency: Latency,
    pub interaction: twilight_model::application::interaction::Interaction,
    pub response: luro_model::builders::InteractionResponseBuilder,
}
