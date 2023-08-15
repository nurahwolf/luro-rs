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

use crate::{luro_message_source::LuroMessageSource, slash_user::SlashUser};

/// Effectively a wrapper around different type of messages, for more streamlined responses
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct LuroMessage {
    #[serde(default)]
    pub author: Option<User>,
    #[serde(default)]
    pub author_id: Option<Id<UserMarker>>,
    #[serde(default)]
    pub member: Option<Member>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub guild_id: Option<Id<GuildMarker>>,
    pub channel_id: Id<ChannelMarker>,
    pub id: Id<MessageMarker>,
    pub source: LuroMessageSource,
    #[serde(default)]
    pub embeds: Option<Vec<Embed>>,
    #[serde(default)]
    pub luro_user: Option<SlashUser>,
    #[serde(default)]
    pub partial_member: Option<PartialMember>,
    #[serde(default)]
    pub message: Option<Message>
}

impl LuroMessage {
    /// This function mutates the reference and overrides the user information.
    ///
    /// Cached messages are a little limited in the information they contain.
    /// While the from trait can be used, using this function includes some extra user information.
    pub fn add_user(&mut self, user: &User) -> &mut Self {
        self.author_id = Some(user.id);
        self.author = Some(user.clone());
        self.luro_user = Some(user.into());

        self
    }

    pub fn add_member(&mut self, user: &User, member: &Member, guild_id: &Id<GuildMarker>) -> &mut Self {
        self.author_id = Some(user.id);
        self.author = Some(user.clone());
        self.luro_user = Some(user.into());
        self.member = Some(member.clone());
        self.guild_id = Some(*guild_id);

        self
    }

    pub fn add_partialmember(&mut self, user: &User, partial: &PartialMember) -> &mut Self {
        self.author_id = Some(user.id);
        self.author = Some(user.clone());
        self.luro_user = Some(user.into());
        self.partial_member = Some(partial.clone());

        self
    }
}

impl From<MessageUpdate> for LuroMessage {
    fn from(message: MessageUpdate) -> Self {
        let luro_user = message.author.as_ref().map(SlashUser::from);

        Self {
            author_id: None,
            author: message.author,
            channel_id: message.channel_id,
            content: message.content,
            embeds: message.embeds,
            guild_id: message.guild_id,
            id: message.id,
            luro_user,
            member: None,
            partial_member: None,
            source: LuroMessageSource::MessageUpdate,
            message: None
        }
    }
}

impl From<MessageDelete> for LuroMessage {
    fn from(message: MessageDelete) -> Self {
        Self {
            author_id: None,
            author: None,
            channel_id: message.channel_id,
            content: None,
            embeds: None,
            guild_id: message.guild_id,
            id: message.id,
            luro_user: None,
            member: None,
            partial_member: None,
            source: LuroMessageSource::MessageDelete,
            message: None
        }
    }
}

impl From<MessageCreate> for LuroMessage {
    fn from(message: MessageCreate) -> Self {
        Self {
            author_id: Some(message.author.id),
            author: Some(message.author.clone()),
            channel_id: message.channel_id,
            content: Some(message.content.clone()),
            embeds: Some(message.embeds.clone()),
            guild_id: message.guild_id,
            id: message.id,
            luro_user: Some(SlashUser::from(message.0.author)),
            member: None,
            partial_member: None,
            source: LuroMessageSource::MessageCreate,
            message: None
        }
    }
}

impl From<CachedMessage> for LuroMessage {
    fn from(message: CachedMessage) -> Self {
        Self {
            author_id: Some(message.author()),
            author: None,
            channel_id: message.channel_id(),
            content: Some(message.content().to_owned()),
            embeds: Some(message.embeds().to_vec()),
            guild_id: message.guild_id(),
            id: message.id(),
            luro_user: None,
            member: None,
            partial_member: message.member().cloned(),
            source: LuroMessageSource::CachedMessage,
            message: None
        }
    }
}

impl From<Message> for LuroMessage {
    fn from(message: Message) -> Self {
        Self {
            message: Some(message.clone()),
            author_id: Some(message.author.id),
            author: Some(message.author.clone()),
            channel_id: message.channel_id,
            content: Some(message.content),
            embeds: Some(message.embeds),
            guild_id: message.guild_id,
            id: message.id,
            luro_user: Some(message.author.into()),
            member: None,
            partial_member: message.member,
            source: LuroMessageSource::Message
        }
    }
}
