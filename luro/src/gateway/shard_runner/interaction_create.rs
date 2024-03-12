use twilight_gateway::MessageSender;
use twilight_model::gateway::payload::incoming::InteractionCreate;

use crate::{
    builders::InteractionResponseBuilder,
    gateway::{GatewayArc, GatewayResult},
    models::interaction::InteractionContext,
};

pub async fn interaction_create(
    gw: GatewayArc,
    sh: MessageSender,
    int: Box<InteractionCreate>,
) -> GatewayResult {
    #[cfg(feature = "module-interactions")]
    let framework = InteractionContext {
        gateway: gw,
        shard: sh,
        interaction: int.0,
        response: InteractionResponseBuilder::default(),
    };

    #[cfg(feature = "module-interactions")]
    crate::commands::interaction_handler(framework).await;

    #[cfg(not(feature = "module-interactions"))]
    tracing::warn!(
        "Interaction was received by the framework, but module-interactions is disabled!"
    );
    Ok(())
}
