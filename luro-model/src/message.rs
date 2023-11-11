use serde::{Deserialize, Serialize};
use twilight_model::{
    channel::{
        message::{
            sticker::MessageSticker, Component, Embed, Mention, MessageActivity, MessageApplication, MessageFlags, MessageInteraction,
            MessageReference, MessageType, Reaction, RoleSubscriptionData,
        },
        Attachment, Channel, ChannelMention,
    },
    gateway::payload::incoming::{MessageCreate, MessageDelete, MessageUpdate},
    guild::PartialMember,
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, MessageMarker, RoleMarker, WebhookMarker},
        Id,
    },
    util::Timestamp,
};

use crate::{builders::EmbedBuilder, types::{MessageData, User}};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum MessageSource {
    /// Created from an existing message
    TwilightMessage,
    /// Added / crafted manually
    Custom,
    /// Created from a cached message
    CachedMessage,
    /// Created from a message update event
    MessageUpdate,
    /// Created from a message delete event
    MessageDelete,
    /// Created from a message create event
    MessageCreate,
    /// No message :(
    #[default]
    None,
}

/// Effectively a wrapper around different type of messages, for more streamlined responses
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Message {
    /// Data that is present if fetched from the database
    pub data: Option<MessageData>,
    // Enable this if you need to migrate
    // #[serde(default = "default_user", deserialize_with = "deserialize_user_to_id")]
    pub author: User,
    pub member: Option<PartialMember>,
    /// Present with Rich Presence-related chat embeds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity: Option<MessageActivity>,
    /// Present with Rich Presence-related chat embeds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<MessageApplication>,
    /// Associated application's ID.
    ///
    /// Present if the message is a response to an [`Interaction`].
    ///
    /// [`Interaction`]: crate::application::interaction::Interaction
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Id<ApplicationMarker>>,
    /// List of attachments.
    ///
    /// Receiving the attachments of messages requires that the
    /// [Message Content Intent] be enabled for the application. In the case of
    /// receiving messages over the Gateway, the intent must also be enabled for
    /// the session.
    ///
    /// Message attachments will be empty unless the [Message Content Intent] is
    /// enabled, the message was sent by the current user, or the message is in
    /// a direct message channel.
    ///
    /// [Message Content Intent]: crate::gateway::Intents::MESSAGE_CONTENT
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<Attachment>,
    /// ID of the [`Channel`] the message was sent in.
    pub channel_id: Id<ChannelMarker>,
    /// List of provided components, such as buttons.
    ///
    /// Receiving the components of messages requires that the
    /// [Message Content Intent] be enabled for the application. In the case of
    /// receiving messages over the Gateway, the intent must also be enabled for
    /// the session.
    ///
    /// Message components will be empty unless the [Message Content Intent] is
    /// enabled, the message was sent by the current user, or the message is in
    /// a direct message channel.
    ///
    /// [Message Content Intent]: crate::gateway::Intents::MESSAGE_CONTENT
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<Component>,
    /// Content of the message.
    ///
    /// Receiving the content of messages requires that the
    /// [Message Content Intent] be enabled for the application. In the case of
    /// receiving messages over the Gateway, the intent must also be enabled for
    /// the session.
    ///
    /// Message content will be empty unless the [Message Content Intent] is
    /// enabled, the message was sent by the current user, or the message is in
    /// a direct message channel.
    ///
    /// [Message Content Intent]: crate::gateway::Intents::MESSAGE_CONTENT
    #[serde(default)]
    pub content: String,
    /// When the message was last edited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edited_timestamp: Option<Timestamp>,
    /// List of embeds.
    ///
    /// Receiving the embeds of messages requires that the
    /// [Message Content Intent] be enabled for the application. In the case of
    /// receiving messages over the Gateway, the intent must also be enabled for
    /// the session.
    ///
    /// Message embeds will be empty unless the [Message Content Intent] is
    /// enabled, the message was sent by the current user, or the message is in
    /// a direct message channel.
    ///
    /// [Message Content Intent]: crate::gateway::Intents::MESSAGE_CONTENT
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<Embed>,
    /// Flags of the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    /// ID of the [`Guild`] the message was sent in.
    ///
    /// [`Guild`]: crate::guild::Guild
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// Id of the message.
    pub id: Id<MessageMarker>,
    /// Interaction the message was sent as a response to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interaction: Option<MessageInteraction>,
    /// Type of message.
    pub kind: MessageType,
    /// [`Channel`]s mentioned in the message.
    ///
    /// Note: only textual channels visible to everyone mentioned in crossposted
    /// messages (via channel following) will be included.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mention_channels: Vec<ChannelMention>,
    /// Whether the message mentions `@everyone`.
    #[serde(default)]
    pub mention_everyone: bool,
    /// [`Role`]s mentioned in the message.
    ///
    /// [`Role`]: crate::guild::Role
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mention_roles: Vec<Id<RoleMarker>>,
    /// Users mentioned in the message.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mentions: Vec<Mention>,
    /// Whether the message is pinned.
    #[serde(default)]
    pub pinned: bool,
    /// List of reactions to the message.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reactions: Vec<Reaction>,
    /// Crosspost, channel follow add, pin and reply source message data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<MessageReference>,
    /// The message associated with the [`reference`].
    ///
    /// [`reference`]: Self::reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referenced_message: Option<Box<Message>>,
    /// Information about the role subscription purchase or renewal that
    /// prompted this message.
    ///
    /// Applies to [`RoleSubscriptionPurchase`] messages.
    ///
    /// [`RoleSubscriptionPurchase`]: MessageType::RoleSubscriptionPurchase
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_subscription_data: Option<RoleSubscriptionData>,
    /// Stickers within the message.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sticker_items: Vec<MessageSticker>,
    /// Timestamp of when the message was created.
    pub timestamp: Timestamp,
    /// Thread started from this message, includes [`Channel::member`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread: Option<Channel>,
    /// Whether the message was a TTS message.
    #[serde(default)]
    pub tts: bool,
    /// ID of the webhook that generated the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<Id<WebhookMarker>>,
    /// How was this message created?
    #[serde(default)]
    pub source: MessageSource,
}

impl From<Message> for twilight_model::channel::Message {
    fn from(message: Message) -> Self {
        Self {
            activity: message.activity,
            application: message.application,
            application_id: message.application_id,
            attachments: message.attachments,
            author: message.author.into(),
            channel_id: message.channel_id,
            components: message.components,
            content: message.content,
            edited_timestamp: message.edited_timestamp,
            embeds: message.embeds,
            flags: message.flags,
            guild_id: message.guild_id,
            id: message.id,
            interaction: message.interaction,
            kind: message.kind,
            member: message.member,
            mention_channels: message.mention_channels,
            mention_everyone: message.mention_everyone,
            mention_roles: message.mention_roles,
            mentions: message.mentions,
            pinned: message.pinned,
            reactions: message.reactions,
            reference: message.reference,
            referenced_message: None,
            role_subscription_data: message.role_subscription_data,
            sticker_items: message.sticker_items,
            timestamp: message.timestamp,
            thread: message.thread,
            tts: message.tts,
            webhook_id: message.webhook_id,
        }
    }
}

impl From<twilight_model::channel::Message> for Message {
    fn from(message: twilight_model::channel::Message) -> Self {
        Self {
            data: None,
            source: MessageSource::TwilightMessage,
            activity: message.activity,
            application: message.application,
            application_id: message.application_id,
            attachments: message.attachments,
            author: message.author.into(),
            channel_id: message.channel_id,
            components: message.components,
            content: message.content,
            edited_timestamp: message.edited_timestamp,
            embeds: message.embeds,
            flags: message.flags,
            guild_id: message.guild_id,
            id: message.id,
            interaction: message.interaction,
            kind: message.kind,
            member: message.member,
            mention_channels: message.mention_channels,
            mention_everyone: message.mention_everyone,
            mention_roles: message.mention_roles,
            mentions: message.mentions,
            pinned: message.pinned,
            reactions: message.reactions,
            reference: message.reference,
            referenced_message: None,
            role_subscription_data: message.role_subscription_data,
            sticker_items: message.sticker_items,
            timestamp: message.timestamp,
            thread: message.thread,
            tts: message.tts,
            webhook_id: message.webhook_id,
        }
    }
}

impl Message {
    /// Return a link in the format of `https://discord.com/channels/{guild_id}/{channel_id}/{message_id}`.
    pub fn link(&self) -> String {
        match self.guild_id {
            Some(guild_id) => format!("https://discord.com/channels/{guild_id}/{}/{}", self.channel_id, self.id),
            None => format!("https://discord.com/channels/@me/{}/{}", self.channel_id, self.id),
        }
    }

    pub fn new(id: Id<MessageMarker>, author: User, channel_id: Id<ChannelMarker>, timestamp: Timestamp) -> Self {
        Self {
            source: MessageSource::Custom,
            data: None,
            activity: Default::default(),
            application_id: Default::default(),
            application: Default::default(),
            attachments: Default::default(),
            author,
            channel_id,
            components: Default::default(),
            content: Default::default(),
            edited_timestamp: Default::default(),
            embeds: Default::default(),
            flags: Default::default(),
            guild_id: Default::default(),
            id,
            interaction: Default::default(),
            kind: MessageType::Regular,
            mention_channels: Default::default(),
            mention_everyone: Default::default(),
            mention_roles: Default::default(),
            mentions: Default::default(),
            pinned: Default::default(),
            reactions: Default::default(),
            reference: Default::default(),
            referenced_message: Default::default(),
            role_subscription_data: Default::default(),
            sticker_items: Default::default(),
            thread: Default::default(),
            timestamp,
            member: Default::default(),
            tts: Default::default(),
            webhook_id: Default::default(),
        }
    }

    /// Update this message from a [Message]
    pub fn from_message(&mut self, message: Message) -> &mut Self {
        self.thread = message.thread;
        self.activity = message.activity;
        self.application = message.application;
        self.application_id = message.application_id;
        self.attachments = message.attachments;
        self.author = message.author;
        self.channel_id = message.channel_id;
        self.components = message.components;
        self.content = message.content;
        self.edited_timestamp = message.edited_timestamp;
        self.embeds = message.embeds;
        self.flags = message.flags;
        self.guild_id = message.guild_id;
        self.id = message.id;
        self.referenced_message = message.referenced_message;
        self.interaction = message.interaction;
        self.kind = message.kind;
        self.mention_channels = message.mention_channels;
        self.mention_everyone = message.mention_everyone;
        self.mention_roles = message.mention_roles;
        self.mentions = message.mentions;
        self.pinned = message.pinned;
        self.reactions = message.reactions;
        self.reference = message.reference;
        self.sticker_items = message.sticker_items;
        self.source = MessageSource::TwilightMessage;
        self.timestamp = message.timestamp;
        self.tts = message.tts;
        self.webhook_id = message.webhook_id;
        self
    }

    /// Update this message from a [Message]
    pub fn from_message_update(&mut self, message: MessageUpdate) -> &mut Self {
        if let Some(author) = message.author {
            self.author = author.into()
        }

        if let Some(kind) = message.kind {
            self.kind = kind
        }

        if let Some(mentions) = message.mentions {
            self.mentions = mentions
        }

        if let Some(timestamp) = message.timestamp {
            self.timestamp = timestamp
        }

        self.source = MessageSource::MessageUpdate;
        self.attachments = message.attachments.unwrap_or_default();
        self.channel_id = message.channel_id;
        self.content = message.content.unwrap_or_default();
        self.edited_timestamp = message.edited_timestamp;
        self.embeds = message.embeds.unwrap_or_default();
        self.guild_id = message.guild_id;
        self.id = message.id;
        self.mention_everyone = message.mention_everyone.unwrap_or_default();
        self.mention_roles = message.mention_roles.unwrap_or_default();
        self.pinned = message.pinned.unwrap_or_default();
        self.tts = message.tts.unwrap_or_default();
        self
    }

    pub fn from_message_delete(&mut self, message: MessageDelete) -> &mut Self {
        self.id = message.id;
        self.channel_id = message.channel_id;
        self.guild_id = message.guild_id;
        // self.deleted = true;
        self.source = MessageSource::MessageDelete;
        self
    }

    /// Update this message from a [Message]
    pub fn from_message_create(&mut self, message: MessageCreate) -> &mut Self {
        self.thread = message.0.thread;
        self.activity = message.0.activity;
        self.application = message.0.application;
        self.application_id = message.0.application_id;
        self.attachments = message.0.attachments;
        self.author = message.0.author.into();
        self.channel_id = message.0.channel_id;
        self.components = message.0.components;
        self.content = message.0.content;
        self.edited_timestamp = message.0.edited_timestamp;
        self.embeds = message.0.embeds;
        self.flags = message.0.flags;
        self.guild_id = message.0.guild_id;
        self.id = message.0.id;
        self.interaction = message.0.interaction;
        self.kind = message.0.kind;
        self.mention_channels = message.0.mention_channels;
        self.mention_everyone = message.0.mention_everyone;
        self.mention_roles = message.0.mention_roles;
        self.mentions = message.0.mentions;
        self.pinned = message.0.pinned;
        self.reactions = message.0.reactions;
        self.reference = message.0.reference;
        // TODO: Implement this
        self.referenced_message = None;
        self.source = MessageSource::MessageCreate;
        self.sticker_items = message.0.sticker_items;
        self.timestamp = message.0.timestamp;
        self.tts = message.0.tts;
        self.webhook_id = message.0.webhook_id;
        self
    }

    /// Update this message from a [Message]
    #[cfg(feature = "twilight-cache")]
    pub fn from_cached_message(&mut self, message: CachedMessage) -> &mut Self {
        self.source = MessageSource::TwilightMessage;
        self.activity = message.activity().cloned();
        self.application = message.application().cloned();
        self.application_id = message.application_id();
        self.attachments = message.attachments().to_vec();
        self.channel_id = message.channel_id();
        self.components = message.components().to_vec();
        self.content = message.content().to_string();
        self.edited_timestamp = message.edited_timestamp();
        self.embeds = message.embeds().to_vec();
        self.flags = message.flags();
        self.guild_id = message.guild_id();
        self.id = message.id();
        self.kind = message.kind();
        self.mention_channels = message.mention_channels().to_vec();
        self.mention_everyone = message.mention_everyone();
        self.mention_roles = message.mention_roles().to_vec();
        self.pinned = message.pinned();
        self.reactions = message.reactions().to_vec();
        self.reference = message.reference().cloned();
        self.sticker_items = message.sticker_items().to_vec();
        self.timestamp = message.timestamp();
        self.tts = message.tts();
        self.webhook_id = message.webhook_id();
        self
    }
}

impl From<MessageCreate> for Message {
    fn from(message: MessageCreate) -> Self {
        let mut luro = Self::new(message.id, message.author.clone().into(), message.channel_id, message.timestamp);
        luro.from_message_create(message);
        luro
    }
}

#[cfg(feature = "twilight-cache")]
impl From<CachedMessage> for Message {
    fn from(message: CachedMessage) -> Self {
        let mut luro = Self::new(
            message.id(),
            default_user(message.author()),
            message.channel_id(),
            message.timestamp(),
        );
        luro.from_cached_message(message);
        luro
    }
}


impl Message {
    /// Create and append an embed. Multiple calls will add multiple embeds.
    ///
    /// NOTE: This WILL fail to send if more than 10 embeds are present!
    ///
    /// Refer to the documentation for [`EmbedBuilder`] for more
    /// information.
    pub fn embed<F>(&mut self, embed: F) -> &mut Self
    where
        F: FnOnce(&mut EmbedBuilder) -> &mut EmbedBuilder,
    {
        let mut e = EmbedBuilder::default();
        embed(&mut e);
        self.embeds.push(e.into());

        self
    }

    /// Explicitly set and overwrite all currently set embeds.
    /// Modify the nested embeds field for more advanced controls.
    ///
    /// NOTE: This WILL fail to send if more than 10 are present!
    pub fn set_embeds(&mut self, embeds: Vec<Embed>) -> &mut Self {
        self.embeds = embeds;

        self
    }

    /// Set the content that should be sent with the message.
    /// This will overrwrite anything previously set.
    pub fn content(&mut self, content: impl Into<String>) -> &mut Self {
        self.content = content.into();

        self
    }
}