use anyhow::anyhow;
use core::fmt;
use std::{fmt::Display, mem, str::FromStr};
use twilight_model::user::CurrentUser;
use twilight_util::builder::embed::EmbedBuilder;

use anyhow::{bail, Error};
use twilight_http::client::InteractionClient;
use twilight_model::http::interaction::InteractionResponse;
use twilight_model::{
    application::interaction::{
        application_command::InteractionMember, message_component::MessageComponentInteractionData,
        modal::ModalInteractionData, Interaction, InteractionData,
    },
    channel::Channel,
    guild::PartialMember,
    http::interaction::InteractionResponseType,
};
use twilight_util::builder::InteractionResponseDataBuilder;

/// A function that takes a borred interaction, and returns a borred reference to interaction.channel and a user who invoked the interaction. Additionally it calls a debug to print where the command was executed in the logs
pub fn interaction_context<'a>(
    interaction: &'a Interaction,
    command_name: &str,
) -> anyhow::Result<(&'a Channel, &'a User, Option<&'a PartialMember>)> {
    let invoked_channel = interaction
        .channel
        .as_ref()
        .ok_or_else(|| Error::msg("Unable to get the channel this interaction was ran in"))?;
    let interaction_member = interaction.member.as_ref();
    let interaction_author = match interaction.member.as_ref() {
        Some(member) => member
            .user
            .as_ref()
            .ok_or_else(|| Error::msg("Unable to find the user that executed this command"))?,
        None => interaction
            .user
            .as_ref()
            .ok_or_else(|| Error::msg("Unable to find the user that executed this command"))?,
    };

    match &invoked_channel.name {
        Some(channel_name) => tracing::debug!(
            "'{}' interaction in channel {} by {}",
            command_name,
            channel_name,
            interaction_author.name
        ),
        None => tracing::debug!(
            "'{}' interaction by {}",
            command_name,
            interaction_author.name
        ),
    };

    Ok((invoked_channel, interaction_author, interaction_member))
}

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

use crate::ACCENT_COLOUR;
use crate::{
    interactions::{InteractionResponder, InteractionResponse as LuroResponse},
    LuroContext,
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

/// Return the user's avatar fromH
pub fn get_partial_member_avatar(
    member: Option<&PartialMember>,
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

/// Return the user's avatar fromH
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

/// Return a string that is a link to the user's avatar
pub fn get_currentuser_avatar(currentuser: &CurrentUser) -> String {
    let user_id = currentuser.id;

    if let Some(user_avatar) = currentuser.avatar {
        match user_avatar.is_animated() {
            true => {
                return format!("https://cdn.discordapp.com/avatars/{user_id}/{user_avatar}.gif")
            }
            false => {
                return format!("https://cdn.discordapp.com/avatars/{user_id}/{user_avatar}.png")
            }
        }
    };

    let modulo = currentuser.discriminator % 5;
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

/// Parse incoming [`ModalSubmit`] interaction and return the inner data.
///
/// This takes a mutable [`Interaction`] since the inner [`ModalInteractionData`]
/// is replaced with [`None`] to avoid useless clones.
///
/// [`ModalSubmit`]: twilight_model::application::interaction::InteractionType::ModalSubmit
/// [`ModalInteractionData`]: twilight_model::application::interaction::modal::ModalInteractionData
pub fn parse_modal_data(
    interaction: &mut Interaction,
) -> Result<ModalInteractionData, anyhow::Error> {
    match mem::take(&mut interaction.data) {
        Some(InteractionData::ModalSubmit(data)) => Ok(data),
        _ => bail!("unable to parse modal data, received unknown data type"),
    }
}

pub fn parse_component_data(
    interaction: &mut Interaction,
) -> Result<MessageComponentInteractionData, anyhow::Error> {
    match mem::take(&mut interaction.data) {
        Some(InteractionData::MessageComponent(data)) => Ok(data),
        _ => bail!("unable to parse modal data, received unknown data type"),
    }
}

/// Parse a field from [`ModalInteractionData`].
///
/// This function try to find a field with the given name in the modal data and
/// return its value as a string.
pub fn parse_modal_field<'a>(
    data: &'a ModalInteractionData,
    name: &str,
) -> Result<Option<&'a str>, anyhow::Error> {
    let mut components = data.components.iter().flat_map(|c| &c.components);

    match components.find(|c| &*c.custom_id == name) {
        Some(component) => Ok(component.value.as_deref()),
        None => bail!("missing modal field: {}", name),
    }
}

/// Parse a required field from [`ModalInteractionData`].
///
/// This function is the same as [`parse_modal_field`] but returns an error if
/// the field value is [`None`].
pub fn parse_modal_field_required<'a>(
    data: &'a ModalInteractionData,
    name: &str,
) -> Result<&'a str, anyhow::Error> {
    let value = parse_modal_field(data, name)?;

    value.ok_or_else(|| anyhow!("required modal field is empty: {}", name))
}

pub async fn defer_interaction(ctx: &LuroContext, interaction: &Interaction) -> anyhow::Result<()> {
    InteractionResponder::from_interaction(interaction)
        .respond(
            ctx,
            LuroResponse::Raw {
                kind: InteractionResponseType::DeferredChannelMessageWithSource,
                data: None,
            },
        )
        .await?;

    Ok(())
}

pub async fn accent_colour(ctx: &LuroContext, guild_id: Option<Id<GuildMarker>>) -> u32 {
    if let Some(guild_id) = guild_id {
        let guild_db = ctx.guilds.read();
        let guild_settings = guild_db.get(&guild_id);

        if let Some(guild_settings) = guild_settings {
            if let Some(custom_accent_colour) = guild_settings.accent_colour_custom {
                return custom_accent_colour;
            };

            return guild_settings.accent_colour;
        }
    };

    ACCENT_COLOUR
}

pub async fn base_embed(ctx: &LuroContext, guild_id: Option<Id<GuildMarker>>) -> EmbedBuilder {
    EmbedBuilder::new().color(accent_colour(ctx, guild_id).await)
}
