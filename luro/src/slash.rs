use std::sync::Arc;

use luro_database::TomlDatabaseDriver;
use twilight_gateway::{Latency, MessageSender};
use twilight_model::{
    application::{command::CommandOptionChoice, interaction::Interaction},
    channel::message::{AllowedMentions, Component, Embed, MessageFlags},
    http::{attachment::Attachment, interaction::InteractionResponseType}
};

use crate::framework::Framework;

/// Some nice stuff about formatting a response, ready to send via twilight's client
#[derive(Clone, Debug)]
#[deprecated]
pub struct Slash {
    // /// Luro's context, used for utility such as setting the embed accent colour and for sending our response near the end.
    pub framework: Arc<Framework<TomlDatabaseDriver>>,
    /// Interaction we are handling
    pub interaction: Interaction,
    pub shard: MessageSender,
    /// The interaction response type for our response. Defaults to [`InteractionResponseType::ChannelMessageWithSource`].
    pub interaction_response_type: InteractionResponseType,
    /// Allowed mentions of the response.
    pub allowed_mentions: Option<AllowedMentions>,
    /// List of attachments on the response.
    pub attachments: Option<Vec<Attachment>>,
    /// List of autocomplete alternatives.
    ///
    /// Can only be used with
    /// [`InteractionResponseType::ApplicationCommandAutocompleteResult`].
    pub choices: Option<Vec<CommandOptionChoice>>,
    /// List of components on the response.
    pub components: Option<Vec<Component>>,
    /// Content of the response.
    pub content: Option<String>,
    /// For [`InteractionResponseType::Modal`], user defined identifier.
    pub custom_id: Option<String>,
    /// Embeds of the response.
    pub embeds: Option<Vec<Embed>>,
    /// Interaction response data flags.
    ///
    /// The supported flags are [`MessageFlags::SUPPRESS_EMBEDS`] and
    /// [`MessageFlags::EPHEMERAL`].
    pub flags: Option<MessageFlags>,
    /// For [`InteractionResponseType::Modal`], title of the modal.
    pub title: Option<String>,
    /// Whether the response is TTS.
    pub tts: Option<bool>,
    pub latency: Latency
}
