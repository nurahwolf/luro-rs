use anyhow::Context;
use luro_framework::ComponentInteraction;
use luro_model::types::CommandResponse;
use twilight_model::{application::interaction::Interaction, http::interaction::InteractionResponseType};

use crate::commands::character::components;

pub async fn character_menu(
    ctx: ComponentInteraction,
    invoking_interaction: Interaction,
    state: bool,
) -> anyhow::Result<CommandResponse> {
    let original_author = invoking_interaction
        .author_id()
        .context("Expected to get user ID from interaction")?;
    if ctx.author.user_id != original_author {
        return ctx
            .respond(|r| {
                r.content(format!(
                    "Sorry, only the profile owner <@{original_author}> can change these settings!"
                ))
                .ephemeral()
            })
            .await;
    }

    let embed = ctx.message.embeds.first().context("Expected to find an embed")?;

    ctx.respond(|r| {
        r.response_type(InteractionResponseType::UpdateMessage)
            .set_embed(embed.clone())
            .components(|c| {
                *c = components(state, ctx.channel.nsfw.unwrap_or_default());
                c
            })
    })
    .await
}
