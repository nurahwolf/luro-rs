//! Utility function to handle incoming interactions.

use std::{
    fmt::{self, Display},
    mem,
    str::FromStr,
};

use anyhow::{anyhow, bail, Context};
use tracing::error;
use tracing::instrument;
use twilight_interactions::command::CommandModel;
use twilight_model::{
    application::interaction::{modal::ModalInteractionData, Interaction, InteractionData},
    channel::message::{Component, Embed, MessageFlags},
    guild::PartialMember,
    id::{marker::GuildMarker, Id},
    user::User,
};
use twilight_model::{
    http::interaction::{
        InteractionResponse as HttpInteractionResponse, InteractionResponseData,
        InteractionResponseType,
    },
    id::marker::{ApplicationMarker, InteractionMarker},
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::luro::Luro;

/// Wrapper around [`Interaction`] to provide some utility functions.
#[derive(Debug)]
pub struct InteractionContext {
    /// The wrapped interaction.
    pub interaction: Interaction,
    /// User that invoked the interaction.
    pub author: User,
}

impl InteractionContext {
    /// Create a new [`InteractionContext`] from an [`Interaction`].
    #[instrument]
    pub fn new(interaction: Interaction) -> Result<Self, anyhow::Error> {
        let author = interaction_user(&interaction).context("missing interaction user")?;

        Ok(Self {
            interaction,
            author,
        })
    }
}

fn interaction_user(interaction: &Interaction) -> Option<User> {
    if let Some(member) = &interaction.member {
        if let Some(user) = &member.user {
            return Some(user.clone());
        }
    }

    if let Some(user) = &interaction.user {
        return Some(user.clone());
    }

    None
}

/// Wrapper around an [`Interaction`] that was invoked in a guild.
///
/// This type is similar to [`InteractionContext`], but provides additional
/// fields for guild interactions.
pub struct GuildInteractionContext {
    /// The wrapped interaction.
    pub interaction: Interaction,
    /// User that invoked the interaction.
    pub author: User,
    /// Member object of the user that invoked the interaction.
    pub member: PartialMember,
    /// Id of the guild the interaction was invoked in.
    pub guild_id: Id<GuildMarker>,
}

impl GuildInteractionContext {
    /// Create a new [`GuildInteractionContext`] from an [`Interaction`].
    #[instrument]
    pub fn new(interaction: Interaction) -> Result<Self, anyhow::Error> {
        let member = interaction
            .member
            .clone()
            .context("missing interaction member")?;
        let author = member
            .user
            .clone()
            .context("missing interaction member user")?;
        let guild_id = interaction
            .guild_id
            .context("missing interaction guild id")?;

        Ok(Self {
            interaction,
            author,
            member,
            guild_id,
        })
    }

    // /// Get the [`GuildConfig`] for the guild the interaction was invoked in.
    // pub async fn config(&self, state: &ClusterState) -> Result<GuildConfig, anyhow::Error> {
    //     let config = state
    //         .database
    //         .get_guild_or_create(self.guild_id)
    //         .await
    //         .context("failed to get guild config")?;

    //     Ok(config)
    // }
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

/// Parse incoming [`ApplicationCommand`] or [`ApplicationCommandAutocomplete`]
/// interactions into typed struct.
///
/// This takes a mutable [`Interaction`] since the inner [`CommandData`] is
/// replaced with [`None`] to avoid useless clones.
///
/// [`ApplicationCommand`]: twilight_model::application::interaction::InteractionType::ApplicationCommand
/// [`ApplicationCommandAutocomplete`]: twilight_model::application::interaction::InteractionType::ApplicationCommandAutocomplete
/// [`CommandData`]: twilight_model::application::interaction::application_command::CommandData
pub fn parse_command_data<T>(interaction: &mut Interaction) -> Result<T, anyhow::Error>
where
    T: CommandModel,
{
    let data = match mem::take(&mut interaction.data) {
        Some(InteractionData::ApplicationCommand(data)) => *data,
        _ => bail!("unable to parse command data, received unknown data type"),
    };

    T::from_interaction(data.into()).context("failed to parse command data")
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

/// Implement `handle` method for a command type.
///
/// The generated method will parse the command from an interaction and execute
/// it. The command type must implement [`CommandModel`] and have an `exec`
/// method with the following signature:
///
/// `async fn exec(self, ctx: InteractionContext, state: &ClusterState) -> Result<InteractionResponse, anyhow::Error>`
#[macro_export]
macro_rules! impl_command_handle {
    ($name:path) => {
        impl $name {
            #[::tracing::instrument]
            pub async fn handle(
                mut interaction: ::twilight_model::application::interaction::Interaction,
                state: &$crate::cluster::ClusterState,
            ) -> Result<$crate::interaction::response::InteractionResponse, ::anyhow::Error> {
                let parsed =
                    $crate::interaction::util::parse_command_data::<Self>(&mut interaction)?;
                let ctx = $crate::interaction::util::InteractionContext::new(interaction)?;

                parsed.exec(ctx, state).await
            }
        }
    };
}

/// Implement `handle` method for a guild command type that is only available in
/// guilds.
///
/// This macro is identical to [`impl_command_handle`] except that it will use
/// [`GuildInteractionContext`] instead of [`InteractionContext`].
///
/// The command type must implement [`CommandModel`] and have an `exec` method
/// with the following signature:
///
///`async fn exec(self, ctx: GuildInteractionContext, state: &ClusterState) -> Result<InteractionResponse, anyhow::Error>`
#[macro_export]
macro_rules! impl_guild_command_handle {
    ($name:path) => {
        impl $name {
            #[::tracing::instrument]
            pub async fn handle(
                mut interaction: ::twilight_model::application::interaction::Interaction,
                mut state: $crate::luro::Luro,
            ) -> Result<$crate::util::InteractionResponse, ::anyhow::Error> {
                let parsed = $crate::util::parse_command_data::<Self>(&mut interaction)?;
                let ctx = $crate::util::GuildInteractionContext::new(interaction)?;

                parsed.exec(ctx, state).await
            }
        }
    };
}

/// Credentials used to respond to an interaction.
#[derive(Debug)]
pub struct InteractionResponder {
    /// ID of the interaction.
    pub id: Id<InteractionMarker>,
    /// ID of the associated application.
    pub application_id: Id<ApplicationMarker>,
    /// Token of the command.
    pub token: String,
}

impl InteractionResponder {
    /// Initialize a new [`InteractionResponder`] from an incoming interaction.
    pub fn from_interaction(interaction: &Interaction) -> Self {
        Self {
            id: interaction.id,
            application_id: interaction.application_id,
            token: interaction.token.clone(),
        }
    }

    /// Send a response to an interaction.
    pub async fn respond(&self, state: &Luro, response: InteractionResponse) {
        let client = state.twilight_client.interaction(self.application_id);

        if let Err(error) = client
            .create_response(self.id, &self.token, &response.into_http())
            .await
        {
            error!(error = ?error, "failed to respond to interaction");
        }
    }
}

/// Response to an interaction.
///
/// This enum contains types that can be used to respond to an interaction.
#[derive(Debug, Clone, PartialEq)]
pub enum InteractionResponse {
    /// Respond with an embed.
    #[allow(unused)]
    Embed(Embed),
    /// Respond with an embed sent as ephemeral message.
    EphemeralEmbed(Embed),
    /// Respond with a modal.
    Modal {
        custom_id: String,
        title: String,
        components: Vec<Component>,
    },
    /// Respond with an ephemeral [`DeferredChannelMessageWithSource`] interaction type.
    ///
    /// [`DeferredChannelMessageWithSource`]: InteractionResponseType::DeferredChannelMessageWithSource
    EphemeralDeferredMessage,
    /// Respond with a raw [`HttpInteractionResponse`].
    Raw {
        kind: InteractionResponseType,
        data: Option<InteractionResponseData>,
    },
}

impl InteractionResponse {
    /// Convert the response into a [`HttpInteractionResponse`].
    fn into_http(self) -> HttpInteractionResponse {
        let kind = match self {
            Self::Modal { .. } => InteractionResponseType::Modal,
            Self::EphemeralDeferredMessage => {
                InteractionResponseType::DeferredChannelMessageWithSource
            }
            Self::Raw { kind, .. } => kind,
            _ => InteractionResponseType::ChannelMessageWithSource,
        };

        let data = match self {
            Self::Embed(embed) => Some(
                InteractionResponseDataBuilder::new()
                    .embeds([embed])
                    .build(),
            ),
            Self::EphemeralEmbed(embed) => Some(
                InteractionResponseDataBuilder::new()
                    .embeds([embed])
                    .flags(MessageFlags::EPHEMERAL)
                    .build(),
            ),
            Self::Modal {
                custom_id,
                title,
                components,
            } => Some(
                InteractionResponseDataBuilder::new()
                    .custom_id(custom_id)
                    .title(title)
                    .components(components)
                    .build(),
            ),
            Self::EphemeralDeferredMessage => Some(
                InteractionResponseDataBuilder::new()
                    .flags(MessageFlags::EPHEMERAL)
                    .build(),
            ),
            Self::Raw { data, .. } => data,
        };

        HttpInteractionResponse { kind, data }
    }
}
