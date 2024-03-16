use twilight_gateway::Latency;

use crate::gateway::GatewayArc;

mod accent_colour;
mod acknowledge_interaction;
mod author;
mod author_id;
mod bot;
mod command_name;
mod component_message;
mod fetch_member;
mod fetch_user;
mod guild;
mod interaction_client;
mod respond;
mod response_send;
mod response_update;
mod standard_response;

pub struct InteractionContext {
    pub gateway: GatewayArc,
    pub shard: twilight_gateway::MessageSender,
    pub latency: Latency,
    pub interaction: twilight_model::application::interaction::Interaction,
    pub response: crate::builders::InteractionResponseBuilder,
}
