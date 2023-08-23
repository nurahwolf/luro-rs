use luro_builder::embed::EmbedBuilder;
use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_model::channel::message::embed::EmbedField;

use crate::{InteractionContext, Framework};

pub mod user_action;

/// A wrapper around [EmbedBuilder] to make easy standardised responses
#[derive(Default, Clone)]
pub struct StandardResponse {
    /// The internal embed, if you wish to manipulate it directly
    pub embed: EmbedBuilder
}

impl StandardResponse {
    pub fn new() -> Self {
        Self {
            embed: Default::default(),
        }
    }

    /// Clone the internal embed and return it. Useful for if you don't want to clone it manually.
    /// 
    /// Generally used when the response is reused
    pub fn embed(&self) -> EmbedBuilder {
        self.embed.clone()
    }

    /// Append a field to state if the response was successfully sent in a DM
    pub fn dm_sent(&mut self, success: bool) -> &mut Self {
        match success {
            true => self.embed.create_field("DM Sent", "Successful", true),
            false => self.embed.create_field("DM Sent", "Failed", true)
        };
        self
    }

    /// Create and append a filed directly to the embed
    /// NOTE: If the resulting embed is being sent by Luro, it is checked to make sure we are not over 25 fields.
    /// There is NO check for this in the builder itself!
    pub fn create_field<S: ToString>(&mut self, name: S, value: S, inline: bool) -> &mut Self {
        let field = EmbedField {
            inline,
            name: name.to_string(),
            value: value.to_string()
        };

        self.embed.0.fields.push(field);
        self
    }

    /// Use the build response to respond to an interaction
    pub async fn interaction_response<D: LuroDatabaseDriver>(&self, framework: Framework<D>, ctx: InteractionContext) -> anyhow::Result<()> {
        ctx.respond(framework, |response|response.add_embed(self.embed())).await
    }
}