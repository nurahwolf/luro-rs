use core::fmt;
use std::{fmt::Display, str::FromStr};
use twilight_http::Client;
use twilight_model::guild::{Guild, Role};
use twilight_model::id::marker::{RoleMarker, UserMarker};

use twilight_util::builder::embed::EmbedBuilder;

use anyhow::{bail, Error};
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

use crate::{LuroContext, ACCENT_COLOUR};

/// Component custom id.
///
/// This type is used to hold component identifiers, used in buttons or modals.
/// Each custom id must have a `name` which correspond to the component type,
/// and optionally an `id` used to store component state.
pub struct CustomId {
    /// Name of the component.
    pub name: String,
    /// ID of the component.
    pub id: Option<String>
}

impl CustomId {
    /// Create a new custom id.
    pub fn new(name: impl Into<String>, id: String) -> Self {
        Self {
            name: name.into(),
            id: Some(id)
        }
    }

    /// Create a new custom id with only a name.
    pub fn name(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            id: None
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
                id: Some(id.to_owned())
            }),
            None => Ok(CustomId {
                name: value.to_owned(),
                id: None
            })
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

mod permissions;

/// Compares the position of two roles.
///
/// This type is used to compare positions of two different roles, using the
/// [`Ord`] trait.
///
/// According to [twilight-model documentation]:
///
/// > Roles are primarily ordered by their position in descending order.
/// > For example, a role with a position of 17 is considered a higher role than
/// > one with a position of 12.
/// >
/// > Discord does not guarantee that role positions are positive, unique, or
/// > contiguous. When two or more roles have the same position then the order
/// > is based on the roles’ IDs in ascending order. For example, given two roles
/// > with positions of 10 then a role with an ID of 1 would be considered a
/// > higher role than one with an ID of 20.
///
/// [twilight-model documentation]: https://docs.rs/twilight-model/0.10.2/twilight_model/guild/struct.Role.html#impl-Ord
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoleOrdering {
    pub id: Id<RoleMarker>,
    pub position: i64
}

/// Calculate the permissions of a member with information from the cache.
pub struct LuroPermissions<'a> {
    twilight_client: &'a Client,
    guild_id: Id<GuildMarker>,
    member_id: Id<UserMarker>,
    member_roles: MemberRoles,
    is_owner: bool
}

/// List of resolved roles of a member.
struct MemberRoles {
    /// Everyone role
    pub everyone: Role,
    /// List of roles of the user
    pub roles: Vec<Role>
}

/// Calculate the permissions for a given guild.
pub struct GuildPermissions<'a> {
    twilight_client: &'a Client,
    guild: Guild
}

/// Create a default embed which has the guild's accent colour if available, otherwise falls back to Luro's accent colour
pub fn default_embed(ctx: &LuroContext, guild_id: Option<Id<GuildMarker>>) -> EmbedBuilder {
    EmbedBuilder::new().color(accent_colour(ctx, guild_id))
}

/// Attempts to get the guild's accent colour, else falls back to getting the hardcoded accent colour
pub fn accent_colour(ctx: &LuroContext, guild_id: Option<Id<GuildMarker>>) -> u32 {
    if let Some(guild_id) = guild_id {
        let guild_db = ctx.guild_data.read();
        let guild_settings = guild_db.get(&guild_id);

        if let Some(guild_settings) = guild_settings {
            // Check to see if a custom colour is defined
            if let Some(custom_accent_colour) = guild_settings.accent_colour_custom {
                return custom_accent_colour;
            };

            if guild_settings.accent_colour != 0 {
                return guild_settings.accent_colour
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
