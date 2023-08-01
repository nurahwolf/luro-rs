use twilight_util::builder::embed::EmbedBuilder;

use anyhow::Error;
use twilight_http::client::InteractionClient;
use twilight_model::http::interaction::InteractionResponse;
use twilight_model::{application::interaction::Interaction, http::interaction::InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;

/// A simple function to respond with `ChannelMessageWithSource`
pub async fn respond_to_interaction(
    interaction_client: &InteractionClient<'_>,
    interaction: &Interaction,
    content: String
) -> Result<(), Error> {
    let data = InteractionResponseDataBuilder::new().content(content).build();

    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(data)
    };

    interaction_client
        .create_response(interaction.id, &interaction.token, &response)
        .await?;

    Ok(())
}

use twilight_model::{
    guild::Member,
    id::{marker::GuildMarker, Id},
    user::User
};

use crate::models::LuroFramework;
use crate::ACCENT_COLOUR;

/// Create a default embed which has the guild's accent colour if available, otherwise falls back to Luro's accent colour
pub fn default_embed(ctx: &LuroFramework, guild_id: &Option<Id<GuildMarker>>) -> EmbedBuilder {
    EmbedBuilder::new().color(accent_colour(ctx, guild_id))
}

/// Attempts to get the guild's accent colour, else falls back to getting the hardcoded accent colour
pub fn accent_colour(ctx: &LuroFramework, guild_id: &Option<Id<GuildMarker>>) -> u32 {
    if let Some(guild_id) = guild_id {
        let guild_settings = ctx.guild_data.get(guild_id);

        if let Some(guild_settings) = guild_settings {
            // Check to see if a custom colour is defined
            if let Some(custom_accent_colour) = guild_settings.accent_colour_custom {
                return custom_accent_colour;
            };

            if guild_settings.accent_colour != 0 {
                return guild_settings.accent_colour;
            }
        }
    };

    ACCENT_COLOUR
}

/// Return a string that is a link to the user's avatar
pub fn get_user_avatar(user: &User) -> String {
    let user_id = user.id;

    if let Some(user_avatar) = user.avatar {
        match user_avatar.is_animated() {
            true => return format!("https://cdn.discordapp.com/avatars/{user_id}/{user_avatar}.gif"),
            false => return format!("https://cdn.discordapp.com/avatars/{user_id}/{user_avatar}.png")
        }
    };

    let modulo = user.discriminator % 5;
    format!("https://cdn.discordapp.com/embed/avatars/{modulo}.png")
}

/// Return a string that is a link to the member's avatar, falling back to user avatar if it does not exist
pub fn get_member_avatar(member: Option<&Member>, guild_id: &Option<Id<GuildMarker>>, user: &User) -> String {
    let user_id = user.id;

    if let Some(member) = member && let Some(guild_id) = guild_id && let Some(member_avatar) = member.avatar {
        match member_avatar.is_animated() {
            true => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.gif"),
            false => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.png"),
        }
    };

    get_user_avatar(user)
}
