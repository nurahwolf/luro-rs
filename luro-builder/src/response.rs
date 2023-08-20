use twilight_model::{
    application::command::CommandOptionChoice,
    channel::message::{AllowedMentions, Component, Embed, MentionType, MessageFlags},
    http::{
        attachment::Attachment,
        interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType}
    },
    id::{
        marker::{MessageMarker, StickerMarker},
        Id
    }
};

/// Luro's response builder. This is a nice builder for putting together different type of responses and then sending them in sane ways.
/// When used in Luro, this allows for responding to deferred messages, responding to interactions and creating new messages all from the same parts.
///
/// NOTE: Defaults to [`InteractionResponseType::ChannelMessageWithSource`] if responding to an interaction (non deferred). Use the deferred function to make it deferred.
///
/// Supports:
/// - Responding to an interaction
/// - Sending the response as a message update, if [`InteractionResponseType::DeferredChannelMessageWithSource`] is set.
/// - Sends a new message if no interaction is present.
pub struct LuroResponse {
    /// The type of users that may be mentioned by the bot
    pub allowed_mentions: Option<AllowedMentions>,
    /// List of attachments on the response.
    pub attachments: Option<Vec<Attachment>>,
    /// Content of the response.
    pub content: Option<String>,
    /// List of components on the response.
    pub components: Option<Vec<Component>>,
    /// Generally used for [`InteractionResponseType::Modal`], a user defined identifier.
    pub custom_id: Option<String>,
    /// Embeds of the response.
    pub embeds: Option<Vec<Embed>>,
    /// Specify the type of response we should have
    pub interaction_response_type: InteractionResponseType,
    /// List of autocomplete alternatives.
    ///
    /// Can only be used with
    /// [`InteractionResponseType::ApplicationCommandAutocompleteResult`].
    pub choices: Option<Vec<CommandOptionChoice>>,
    /// Interaction response data flags.
    ///
    /// The supported flags are [`MessageFlags::SUPPRESS_EMBEDS`] and
    /// [`MessageFlags::EPHEMERAL`].
    pub flags: Option<MessageFlags>,
    /// For [`InteractionResponseType::Modal`], title of the modal.
    pub title: Option<String>,
    /// Whether the response is TTS.
    pub tts: Option<bool>,
    /// A message ID in which to reply to
    pub reply: Option<Id<MessageMarker>>,
    /// An array of stickers. Generally only up to 3 can be used at a time.
    pub stickers: Option<Vec<Id<StickerMarker>>>
}

impl Default for LuroResponse {
    fn default() -> Self {
        Self {
            attachments: Default::default(),
            content: Default::default(),
            components: Default::default(),
            custom_id: Default::default(),
            embeds: Default::default(),
            interaction_response_type: InteractionResponseType::ChannelMessageWithSource,
            allowed_mentions: Some(AllowedMentions {
                parse: vec![MentionType::Everyone, MentionType::Roles, MentionType::Users],
                replied_user: false,
                roles: Vec::new(),
                users: Vec::new()
            }),
            choices: Default::default(),
            flags: Default::default(),
            title: Default::default(),
            tts: Default::default(),
            reply: Default::default(),
            stickers: Default::default()
        }
    }
}

mod attachments;
mod choices;
mod components;
mod content;
mod custom_id;
mod embed;
mod response_type;
mod title;

impl LuroResponse {
    /// Returns an ['InteractionResponse'] based on the variables of this structure. Only used for interaction responses.
    pub fn interaction_response(&self) -> InteractionResponse {
        InteractionResponse {
            kind: self.interaction_response_type,
            data: Some(InteractionResponseData {
                allowed_mentions: self.allowed_mentions.clone(),
                attachments: self.attachments.clone(),
                choices: self.choices.clone(),
                components: self.components.clone(),
                content: self.content.clone(),
                custom_id: self.custom_id.clone(),
                embeds: self.embeds.clone(),
                flags: self.flags,
                title: self.title.clone(),
                tts: self.tts
            })
        }
    }

    /// Acknowledges a component interaction and edits the message.
    ///
    /// This is only valid for components and modal submits.
    pub fn update(&mut self) -> &mut Self {
        self.interaction_response_type = InteractionResponseType::UpdateMessage;
        self
    }

    /// Set's the message to reply to a particular message ID.
    /// Responds to the interaction author by default.
    pub fn reply(&mut self, id: &Id<MessageMarker>) -> &mut Self {
        self.reply = Some(*id);
        self
    }

    /// Set the message so that it can only be viewed by the person invoking the command. Has no effect on non-interaction messages.
    pub fn ephemeral(&mut self) -> &mut Self {
        self.flags = Some(MessageFlags::EPHEMERAL);
        self
    }
}
