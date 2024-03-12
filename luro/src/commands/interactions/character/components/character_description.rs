use anyhow::Context;
use luro_framework::{ComponentInteraction, Luro};
use luro_model::types::CommandResponse;
use twilight_model::application::interaction::Interaction;

use crate::commands::character::Character;

impl Character {
    pub async fn character_description_button(
        &self,
        ctx: ComponentInteraction,
        invoking_interaction: Interaction,
    ) -> anyhow::Result<CommandResponse> {
        let nsfw = ctx.channel.nsfw.unwrap_or_default();
        let original_author_id = invoking_interaction
            .author_id()
            .context("Expected to get user ID from interaction")?;
        let original_author = ctx.fetch_user(original_author_id).await?;
        let character_name = self.character_name();
        let character = match ctx.database.user_fetch_character(original_author_id, character_name).await? {
            Some(character) => character,
            None => return ctx.respond(|r|r.content(format!("Sorry, could not find the character {character_name} in my database. The user might have deleted this profile, sorry!")).ephemeral()).await,
        };
        let character_icon = match nsfw {
            true => character.nsfw_icon.as_ref().unwrap_or(&character.sfw_icon),
            false => &character.sfw_icon,
        };

        ctx.respond(|response| {
            {
                response.embed(|embed| {
                    embed
                        .author(|a| {
                            a.icon_url(original_author.avatar_url()).name(format!(
                                "{} [Character by {}]",
                                character.name,
                                original_author.name()
                            ))
                        })
                        .colour(character.colour.unwrap_or(ctx.accent_colour()))
                        .thumbnail(|t| t.url(character_icon))
                        .description(match nsfw {
                            true => format!(
                                "**NSFW Description:**\n{}",
                                character.nsfw_description.unwrap_or(character.sfw_description)
                            ),
                            false => format!("**SFW Description:**\n{}", character.sfw_description),
                        })
                })
            }
            .ephemeral()
        })
        .await
    }
}
