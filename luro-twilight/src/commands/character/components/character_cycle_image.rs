use anyhow::Context;
use luro_framework::{ComponentInteraction, Luro};
use luro_model::types::CommandResponse;
use twilight_model::{application::interaction::Interaction, http::interaction::InteractionResponseType};

use crate::commands::character::character_response;

impl crate::commands::character::Character {
    pub async fn character_cycle_image_button(
        &self,
        ctx: ComponentInteraction,
        invoking_interaction: Interaction,
    ) -> anyhow::Result<CommandResponse> {
        let original_author_id = invoking_interaction
            .author_id()
            .context("Expected to get user ID from interaction")?;
        let original_author = ctx.fetch_user(original_author_id).await?;
        let character_name = self.character_name();
        let character = match ctx.database.user_fetch_character(original_author_id, character_name).await? {
            Some(character) => character,
            None => return ctx.respond(|r|r.content(format!("Sorry, could not find the character {character_name} in my database. The user might have deleted this profile, sorry!")).ephemeral()).await,
        };

        let mut response = character_response(ctx.clone(), &character, &original_author, ctx.channel.nsfw.unwrap_or_default()).await;
        response.response_type(InteractionResponseType::UpdateMessage);
        ctx.response_send(response).await
    }
}
