use core::fmt;
use std::{fmt::Display, str::FromStr};

use anyhow::{bail, Error};
use twilight_http::client::InteractionClient;
use twilight_model::{
    application::interaction::{application_command::InteractionMember, Interaction},
    http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::InteractionResponseDataBuilder;

/// A simple function to respond with `ChannelMessageWithSource`
pub async fn respond_to_interaction(
    interaction_client: &InteractionClient<'_>,
    interaction: &Interaction,
    content: String,
) -> Result<(), Error> {
    let data = InteractionResponseDataBuilder::new()
        .content(content)
        .build();

    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(data),
    };

    interaction_client
        .create_response(interaction.id, &interaction.token, &response)
        .await?;

    Ok(())
}

use twilight_model::{
    guild::Member,
    id::{marker::GuildMarker, Id},
    user::User,
};

pub fn assemble_user_avatar(user: &User) -> String {
    let user_id = user.id;
    user.avatar.map_or_else(
        || {
            format!(
                "https://cdn.discordapp.com/embed/avatars/{}.png",
                user.discriminator % 5
            )
        },
        |avatar| format!("https://cdn.discordapp.com/avatars/{user_id}/{avatar}.png"),
    )
}

/// Return a string that is a link to the member's avatar, falling back to user avatar if it does not exist
pub fn get_interaction_member_avatar(
    member: Option<InteractionMember>,
    guild_id: &Option<Id<GuildMarker>>,
    user: &User,
) -> String {
    let user_id = user.id;

    if let Some(member) = member && let Some(guild_id) = guild_id && let Some(member_avatar) = member.avatar {
        match member_avatar.is_animated() {
            true => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.gif"),
            false => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.png"),
        }
    };

    get_user_avatar(user)
}

/// Return a string that is a link to the member's avatar, falling back to user avatar if it does not exist
pub fn get_member_avatar(
    member: Option<&Member>,
    guild_id: &Option<Id<GuildMarker>>,
    user: &User,
) -> String {
    let user_id = user.id;

    if let Some(member) = member && let Some(guild_id) = guild_id && let Some(member_avatar) = member.avatar {
        match member_avatar.is_animated() {
            true => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.gif"),
            false => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.png"),
        }
    };

    get_user_avatar(user)
}

/// Return a string that is a link to the user's avatar
pub fn get_user_avatar(user: &User) -> String {
    let user_id = user.id;

    if let Some(user_avatar) = user.avatar {
        match user_avatar.is_animated() {
            true => {
                return format!("https://cdn.discordapp.com/avatars/{user_id}/{user_avatar}.gif")
            }
            false => {
                return format!("https://cdn.discordapp.com/avatars/{user_id}/{user_avatar}.png")
            }
        }
    };

    let modulo = user.discriminator % 5;
    format!("https://cdn.discordapp.com/embed/avatars/{modulo}.png")
}

/// Component custom id.
///
/// This type is used to hold component identifiers, used in buttons or modals.
/// Each custom id must have a `name` which correspond to the component type,
/// and optionally an `id` used to store component state.
pub struct CustomId {
    /// Name of the component.
    pub name: String,
    /// ID of the component.
    pub id: Option<String>,
}

impl CustomId {
    /// Create a new custom id.
    pub fn new(name: impl Into<String>, id: String) -> Self {
        Self {
            name: name.into(),
            id: Some(id),
        }
    }

    /// Create a new custom id with only a name.
    pub fn name(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            id: None,
        }
    }
}

impl FromStr for CustomId {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            bail!("expected non-empty custom id");
        }

        match value.split_once(':') {
            Some((name, id)) => Ok(CustomId {
                name: name.to_owned(),
                id: Some(id.to_owned()),
            }),
            None => Ok(CustomId {
                name: value.to_owned(),
                id: None,
            }),
        }
    }
}

impl Display for CustomId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(id) = &self.id {
            f.write_str(&self.name)?;
            f.write_str(":")?;
            f.write_str(id)
        } else {
            f.write_str(&self.name)
        }
    }
}
