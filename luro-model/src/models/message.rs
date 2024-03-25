use serde::{Deserialize, Serialize};
use twilight_model::id::marker::MessageMarker;
use twilight_model::{
    channel::message::{Embed, MessageType},
    gateway::payload::incoming::{MessageCreate, MessageDelete, MessageUpdate},
    id::{marker::ChannelMarker, Id},
    util::Timestamp,
};

use crate::builders::EmbedBuilder;
use crate::user::User;

#[cfg(not(feature = "database-sqlx"))]
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum MessageSource {
    /// Created from an existing message
    Message,
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

#[cfg(feature = "database-sqlx")]
#[derive(Clone, Debug, serde::Deserialize, PartialEq, serde::Serialize, Default, ::sqlx::Type)]
#[sqlx(type_name = "message_source", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MessageSource {
    /// Created from an existing message
    TwilightMessage,
    /// Added / crafted manually
    LuroMessage,
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

impl std::fmt::Display for MessageSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            MessageSource::TwilightMessage => "Twilight Message (Direct from the API)",
            MessageSource::LuroMessage => "Custom Message (Message with custom data)",
            MessageSource::CachedMessage => "Cached Message (Twilight Cache)",
            MessageSource::MessageUpdate => "Message Update",
            MessageSource::MessageDelete => "Message Delete",
            MessageSource::MessageCreate => "Message Create",
            MessageSource::None => "No source present",
        };

        write!(f, "{}", name)
    }
}

/// Effectively a wrapper around different type of messages, for more streamlined responses
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Message {
    pub twilight_message: twilight_model::channel::Message,
    pub updated_content: Option<Vec<MessageUpdate>>,
    /// Has the message been marked as deleted in the database
    #[serde(default)]
    pub deleted: Option<bool>,
    pub source: MessageSource,
}

impl From<Message> for twilight_model::channel::Message {
    fn from(message: Message) -> Self {
        message.twilight_message
    }
}

impl From<twilight_model::channel::Message> for Message {
    fn from(twilight_message: twilight_model::channel::Message) -> Self {
        Self {
            twilight_message,
            deleted: None,
            updated_content: None,
            source: MessageSource::TwilightMessage,
        }
    }
}

impl Message {
    /// Return a link in the format of `https://discord.com/channels/{guild_id}/{channel_id}/{message_id}`.
    pub fn link(&self) -> String {
        match self.twilight_message.guild_id {
            Some(guild_id) => format!(
                "https://discord.com/channels/{guild_id}/{}/{}",
                self.twilight_message.channel_id, self.twilight_message.id
            ),
            None => format!(
                "https://discord.com/channels/@me/{}/{}",
                self.twilight_message.channel_id, self.twilight_message.id
            ),
        }
    }

    pub fn new(id: Id<MessageMarker>, author: User, channel_id: Id<ChannelMarker>, timestamp: Timestamp) -> Self {
        Self {
            source: MessageSource::LuroMessage,
            twilight_message: twilight_model::channel::Message {
                activity: None,
                application_id: None,
                application: None,
                attachments: Default::default(),
                author: author.into(),
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
            },
            updated_content: None,
            deleted: None,
        }
    }

    /// Update this message from a [Message]
    pub fn from_message(&mut self, message: Message) -> &mut Self {
        self.twilight_message.thread = message.twilight_message.thread;
        self.twilight_message.activity = message.twilight_message.activity;
        self.twilight_message.application = message.twilight_message.application;
        self.twilight_message.application_id = message.twilight_message.application_id;
        self.twilight_message.attachments = message.twilight_message.attachments;
        self.twilight_message.author = message.twilight_message.author;
        self.twilight_message.channel_id = message.twilight_message.channel_id;
        self.twilight_message.components = message.twilight_message.components;
        self.twilight_message.content = message.twilight_message.content;
        self.twilight_message.edited_timestamp = message.twilight_message.edited_timestamp;
        self.twilight_message.embeds = message.twilight_message.embeds;
        self.twilight_message.flags = message.twilight_message.flags;
        self.twilight_message.guild_id = message.twilight_message.guild_id;
        self.twilight_message.id = message.twilight_message.id;
        self.twilight_message.referenced_message = message.twilight_message.referenced_message;
        self.twilight_message.interaction = message.twilight_message.interaction;
        self.twilight_message.kind = message.twilight_message.kind;
        self.twilight_message.mention_channels = message.twilight_message.mention_channels;
        self.twilight_message.mention_everyone = message.twilight_message.mention_everyone;
        self.twilight_message.mention_roles = message.twilight_message.mention_roles;
        self.twilight_message.mentions = message.twilight_message.mentions;
        self.twilight_message.pinned = message.twilight_message.pinned;
        self.twilight_message.reactions = message.twilight_message.reactions;
        self.twilight_message.reference = message.twilight_message.reference;
        self.twilight_message.sticker_items = message.twilight_message.sticker_items;
        self.source = MessageSource::TwilightMessage;
        self.twilight_message.timestamp = message.twilight_message.timestamp;
        self.twilight_message.tts = message.twilight_message.tts;
        self.twilight_message.webhook_id = message.twilight_message.webhook_id;
        self
    }

    /// Update this message from a [Message]
    pub fn from_message_update(&mut self, message: MessageUpdate) -> &mut Self {
        // if let Some(author) = message.author {
        //     self.author = author.into()
        // }

        if let Some(kind) = message.kind {
            self.twilight_message.kind = kind
        }

        if let Some(mentions) = message.mentions {
            self.twilight_message.mentions = mentions
        }

        if let Some(timestamp) = message.timestamp {
            self.twilight_message.timestamp = timestamp
        }

        self.source = MessageSource::MessageUpdate;
        self.twilight_message.attachments = message.attachments.unwrap_or_default();
        self.twilight_message.channel_id = message.channel_id;
        self.twilight_message.content = message.content.unwrap_or_default();
        self.twilight_message.edited_timestamp = message.edited_timestamp;
        self.twilight_message.embeds = message.embeds.unwrap_or_default();
        self.twilight_message.guild_id = message.guild_id;
        self.twilight_message.id = message.id;
        self.twilight_message.mention_everyone = message.mention_everyone.unwrap_or_default();
        self.twilight_message.mention_roles = message.mention_roles.unwrap_or_default();
        self.twilight_message.pinned = message.pinned.unwrap_or_default();
        self.twilight_message.tts = message.tts.unwrap_or_default();
        self
    }

    pub fn from_message_delete(&mut self, message: MessageDelete) -> &mut Self {
        self.twilight_message.id = message.id;
        self.twilight_message.channel_id = message.channel_id;
        self.twilight_message.guild_id = message.guild_id;
        // self.deleted = true;
        self.source = MessageSource::MessageDelete;
        self
    }

    /// Update this message from a [Message]
    pub fn from_message_create(&mut self, message: MessageCreate) -> &mut Self {
        self.twilight_message.thread = message.0.thread;
        self.twilight_message.activity = message.0.activity;
        self.twilight_message.application = message.0.application;
        self.twilight_message.application_id = message.0.application_id;
        self.twilight_message.attachments = message.0.attachments;
        self.twilight_message.author = message.0.author.into();
        self.twilight_message.channel_id = message.0.channel_id;
        self.twilight_message.components = message.0.components;
        self.twilight_message.content = message.0.content;
        self.twilight_message.edited_timestamp = message.0.edited_timestamp;
        self.twilight_message.embeds = message.0.embeds;
        self.twilight_message.flags = message.0.flags;
        self.twilight_message.guild_id = message.0.guild_id;
        self.twilight_message.id = message.0.id;
        self.twilight_message.interaction = message.0.interaction;
        self.twilight_message.kind = message.0.kind;
        self.twilight_message.mention_channels = message.0.mention_channels;
        self.twilight_message.mention_everyone = message.0.mention_everyone;
        self.twilight_message.mention_roles = message.0.mention_roles;
        self.twilight_message.mentions = message.0.mentions;
        self.twilight_message.pinned = message.0.pinned;
        self.twilight_message.reactions = message.0.reactions;
        self.twilight_message.reference = message.0.reference;
        self.source = MessageSource::MessageCreate;
        self.twilight_message.sticker_items = message.0.sticker_items;
        self.twilight_message.timestamp = message.0.timestamp;
        self.twilight_message.tts = message.0.tts;
        self.twilight_message.webhook_id = message.0.webhook_id;
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
        let mut luro = Self::new(
            message.id,
            User::User(message.author.clone().into()),
            message.channel_id,
            message.timestamp,
        );
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
    pub fn embed<F: FnOnce(&mut EmbedBuilder) -> &mut EmbedBuilder>(&mut self, embed: F) -> &mut Self {
        let mut e = EmbedBuilder::default();
        embed(&mut e);
        self.twilight_message.embeds.push(e.into());

        self
    }

    /// Explicitly set and overwrite all currently set embeds.
    /// Modify the nested embeds field for more advanced controls.
    ///
    /// NOTE: This WILL fail to send if more than 10 are present!
    pub fn set_embeds(&mut self, embeds: Vec<Embed>) -> &mut Self {
        self.twilight_message.embeds = embeds;

        self
    }

    /// Set the content that should be sent with the message.
    /// This will overrwrite anything previously set.
    pub fn content(&mut self, content: impl Into<String>) -> &mut Self {
        self.twilight_message.content = content.into();

        self
    }
}
