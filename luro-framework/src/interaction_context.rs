use luro_builder::embed::EmbedBuilder;
use luro_model::{database::drivers::LuroDatabaseDriver, response::LuroResponse};
use twilight_model::channel::Message;

use crate::Framework;

pub trait LuroInteraction {
    /// Create a response to an interaction.
    /// This automatically handles if the interaction had been deferred.
    async fn respond<D, F>(&self, ctx: &Framework<D>, response: F) -> anyhow::Result<Option<Message>>
    where
        D: LuroDatabaseDriver,
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse;

    /// Create a response. This is used for sending a response to an interaction, as well as to defer interactions.
    /// This CANNOT be used to update a response! Use `response_update` for that!
    async fn response_create<D: LuroDatabaseDriver>(
        &self,
        ctx: &Framework<D>,
        response: &LuroResponse
    ) -> anyhow::Result<Option<Message>>;

    /// Update an existing response
    async fn response_update<D: LuroDatabaseDriver>(
        &self,
        framework: &Framework<D>,
        response: &LuroResponse
    ) -> anyhow::Result<Message>;

    /// Send an existing response builder a response to an interaction.
    /// This automatically handles if the interaction had been deferred.
    async fn send_response<D: LuroDatabaseDriver>(
        &self,
        ctx: &Framework<D>,
        response: LuroResponse
    ) -> anyhow::Result<Option<Message>>;

    /// Create a default embed which has the guild's accent colour if available, otherwise falls back to Luro's accent colour
    async fn default_embed<D: LuroDatabaseDriver>(&self, ctx: &Framework<D>) -> EmbedBuilder;

    /// Attempts to get the guild's accent colour, else falls back to getting the hardcoded accent colour
    async fn accent_colour<D: LuroDatabaseDriver>(&self, ctx: &Framework<D>) -> u32;
}
