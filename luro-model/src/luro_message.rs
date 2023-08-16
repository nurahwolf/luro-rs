use serde::{Deserialize, Serialize};
use twilight_cache_inmemory::model::CachedMessage;
use twilight_model::{
    channel::{message::Embed, Message},
    gateway::payload::incoming::{MessageCreate, MessageDelete, MessageUpdate},
    guild::{Member, PartialMember},
    id::{
        marker::{ChannelMarker, GuildMarker, MessageMarker, UserMarker},
        Id
    },
    user::User
};

use crate::{luro_message_source::LuroMessageSource, luro_user::LuroUser};

/// Effectively a wrapper around different type of messages, for more streamlined responses
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LuroMessage {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub author_id: Option<Id<UserMarker>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub guild_id: Option<Id<GuildMarker>>,
    pub channel_id: Id<ChannelMarker>,
    pub id: Id<MessageMarker>,
    #[serde(default)]
    pub source: LuroMessageSource,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub embeds: Vec<Embed>,
    #[serde(default)]
    pub user: LuroUser,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub message: Option<Message>
}

impl LuroMessage {
    /// Return a link in the format of `https://discord.com/channels/{guild_id}/{channel_id}/{message_id}`.
    pub fn link(&self) -> String {
        match self.guild_id {
            Some(guild_id) => format!("https://discord.com/channels/{guild_id}/{}/{}", self.channel_id, self.id),
            None => format!("https://discord.com/channels/@me/{}/{}", self.channel_id, self.id)
        }
    }

    /// This function mutates the reference and overrides the user information.
    ///
    /// Cached messages are a little limited in the information they contain.
    /// While the from trait can be used, using this function includes some extra user information.
    pub fn add_user(&mut self, user: &User) -> &mut Self {
        self.author_id = Some(user.id);
        self.user = LuroUser::from(user);
        self
    }

    pub fn add_member(&mut self, user: &User, guild_id: &Id<GuildMarker>, member: &Member) -> &mut Self {
        self.author_id = Some(user.id);
        let mut luro_user = LuroUser::from(user);
        luro_user.update_member(guild_id, member);
        self.user = luro_user;
        self.guild_id = Some(*guild_id);

        self
    }

    pub fn add_partialmember(&mut self, user: &User, guild_id: &Id<GuildMarker>, partial: &PartialMember) -> &mut Self {
        self.author_id = Some(user.id);
        let mut luro_user = LuroUser::from(user);
        luro_user.update_partialmember(guild_id, partial);
        self.user = luro_user;
        self.guild_id = Some(*guild_id);
        self
    }
}

impl From<MessageUpdate> for LuroMessage {
    fn from(message: MessageUpdate) -> Self {
        let user = match message.author {
            Some(author) => LuroUser::from(&author),
            None => LuroUser::default()
        };

        Self {
            author_id: None,
            channel_id: message.channel_id,
            content: message.content,
            embeds: message.embeds.unwrap_or_default(),
            guild_id: message.guild_id,
            id: message.id,
            user,
            source: LuroMessageSource::MessageUpdate,
            message: None
        }
    }
}

impl From<MessageDelete> for LuroMessage {
    fn from(message: MessageDelete) -> Self {
        Self {
            author_id: None,
            channel_id: message.channel_id,
            content: None,
            guild_id: message.guild_id,
            id: message.id,
            user: LuroUser::default(),
            source: LuroMessageSource::MessageDelete,
            message: None,
            embeds: vec![]
        }
    }
}

impl From<MessageCreate> for LuroMessage {
    fn from(message: MessageCreate) -> Self {
        let mut user = LuroUser::from(&message.author);
        if let Some(guild_id) = message.guild_id && let Some(ref member) = message.member {
            user.update_partialmember(&guild_id, member);
        }

        Self {
            author_id: Some(message.author.id),
            channel_id: message.channel_id,
            content: Some(message.content.clone()),
            embeds: message.embeds.clone(),
            guild_id: message.guild_id,
            id: message.id,
            user,
            source: LuroMessageSource::MessageCreate,
            message: None
        }
    }
}

impl From<CachedMessage> for LuroMessage {
    fn from(message: CachedMessage) -> Self {
        Self {
            author_id: Some(message.author()),
            channel_id: message.channel_id(),
            content: Some(message.content().to_owned()),
            embeds: message.embeds().to_vec(),
            guild_id: message.guild_id(),
            id: message.id(),
            user: LuroUser::default(),
            source: LuroMessageSource::CachedMessage,
            message: None
        }
    }
}

impl From<Message> for LuroMessage {
    fn from(message: Message) -> Self {
        let mut user = LuroUser::from(&message.author);
        if let Some(guild_id) = message.guild_id && let Some(ref member) = message.member {
            user.update_partialmember(&guild_id, member);
        }
        Self {
            message: Some(message.clone()),
            author_id: Some(message.author.id),
            channel_id: message.channel_id,
            content: Some(message.content),
            embeds: message.embeds,
            guild_id: message.guild_id,
            id: message.id,
            user,
            source: LuroMessageSource::Message
        }
    }
}
