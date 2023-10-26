use serde::{Serialize, Deserialize};
use sqlx::types::Json;
use time::OffsetDateTime;
use twilight_model::{
    channel::{
        forum::{DefaultReaction, ForumLayout, ForumSortOrder, ForumTag},
        permission_overwrite::PermissionOverwrite,
        thread::{AutoArchiveDuration, ThreadMember, ThreadMetadata},
        Channel, ChannelFlags, ChannelType, VideoQualityMode,
    },
    id::{
        marker::{ApplicationMarker, ChannelMarker, GenericMarker, GuildMarker, TagMarker, UserMarker},
        Id,
    },
    user::User,
    util::{ImageHash, Timestamp},
};

use crate::DbChannel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuroChannel {
    /// ID of the application that created the channel.
    pub application_id: Option<Id<ApplicationMarker>>,
    pub applied_tags: Option<Vec<Id<TagMarker>>>,
    pub available_tags: Option<Vec<ForumTag>>,
    /// Bitrate (in bits) setting of audio channels.
    pub bitrate: Option<u32>,
    /// The channel was deleted, a cached instance is being returned from the database
    pub deleted: bool,
    /// Default duration without messages before the channel's threads
    /// automatically archive.
    ///
    /// Automatic archive durations are not locked behind the guild's boost
    /// level.
    pub default_auto_archive_duration: Option<AutoArchiveDuration>,
    /// Default forum layout view used to display posts in forum channels.
    pub default_forum_layout: Option<ForumLayout>,
    pub default_reaction_emoji: Option<DefaultReaction>,
    /// Default sort order used to display posts in forum channels.
    pub default_sort_order: Option<ForumSortOrder>,
    pub default_thread_rate_limit_per_user: Option<u16>,
    /// Flags of the channel.
    pub flags: Option<ChannelFlags>,
    /// ID of the guild the channel is in.
    pub guild_id: Option<Id<GuildMarker>>,
    /// Hash of the channel's icon.
    pub icon: Option<ImageHash>,
    /// ID of the channel.
    pub id: Id<ChannelMarker>,
    /// Whether users can be invited.
    pub invitable: Option<bool>,
    /// Type of the channel.
    ///
    /// This can be used to determine what fields *might* be available.
    pub kind: ChannelType,
    /// For text channels, this is the ID of the last message sent in the
    /// channel.
    ///
    /// For forum channels, this is the ID of the last created thread in the
    /// forum.
    pub last_message_id: Option<Id<GenericMarker>>,
    /// ID of the last message pinned in the channel.
    pub last_pin_timestamp: Option<Timestamp>,
    /// Whether the channel is managed by an application via the [`gdm.join`]
    /// oauth scope.
    ///
    /// This is only applicable to [group channels].
    ///
    /// [`gdm.join`]: crate::oauth::scope::GDM_JOIN
    /// [group channels]: ChannelType::Group
    pub managed: Option<bool>,
    /// Member that created the channel.
    pub member: Option<ThreadMember>,
    /// Number of members in the channel.
    ///
    /// At most a value of 50 is provided although the real number may be
    /// higher.
    pub member_count: Option<i8>,
    /// Number of messages in the channel.
    pub message_count: Option<u32>,
    /// Name of the channel.
    pub name: Option<String>,
    /// Whether a thread was newly created.
    pub newly_created: Option<bool>,
    /// Whether the channel has been configured to be NSFW.
    pub nsfw: Option<bool>,
    /// ID of the creator of the channel.
    pub owner_id: Option<Id<UserMarker>>,
    /// ID of the parent channel.
    ///
    /// For guild channels this is the ID of the parent category channel.
    ///
    /// For threads this is the ID of the channel the thread was created in.
    pub parent_id: Option<Id<ChannelMarker>>,
    /// Explicit permission overwrites for members and roles.
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    /// Sorting position of the channel.
    pub position: Option<i32>,
    /// Amount of seconds a user has to wait before sending another message.
    pub rate_limit_per_user: Option<u16>,
    /// Recipients of the channel.
    pub recipients: Option<Vec<User>>,
    /// ID of the voice region for the channel.
    ///
    /// Defaults to automatic for applicable channels.
    pub rtc_region: Option<String>,
    /// Metadata about a thread.
    pub thread_metadata: Option<ThreadMetadata>,
    /// Topic of the channel.
    pub topic: Option<String>,
    /// Number of users that may be in the channel.
    ///
    /// Zero refers to no limit.
    pub user_limit: Option<u32>,
    /// Camera video quality mode of the channel.
    ///
    /// Defaults to [`VideoQualityMode::Auto`] for applicable channels.
    pub video_quality_mode: Option<VideoQualityMode>,
}

impl From<LuroChannel> for Channel {
    fn from(twilight_channel: LuroChannel) -> Self {
        Self {
            application_id: twilight_channel.application_id,
            applied_tags: twilight_channel.applied_tags,
            available_tags: twilight_channel.available_tags,
            bitrate: twilight_channel.bitrate,
            default_auto_archive_duration: twilight_channel.default_auto_archive_duration,
            default_forum_layout: twilight_channel.default_forum_layout,
            default_reaction_emoji: twilight_channel.default_reaction_emoji,
            default_sort_order: twilight_channel.default_sort_order,
            default_thread_rate_limit_per_user: twilight_channel.default_thread_rate_limit_per_user,
            flags: twilight_channel.flags,
            guild_id: twilight_channel.guild_id,
            icon: twilight_channel.icon,
            id: twilight_channel.id,
            invitable: twilight_channel.invitable,
            kind: twilight_channel.kind,
            last_message_id: twilight_channel.last_message_id,
            last_pin_timestamp: twilight_channel.last_pin_timestamp,
            managed: twilight_channel.managed,
            member: twilight_channel.member,
            member_count: twilight_channel.member_count,
            message_count: twilight_channel.message_count,
            name: twilight_channel.name,
            newly_created: twilight_channel.newly_created,
            nsfw: twilight_channel.nsfw,
            owner_id: twilight_channel.owner_id,
            parent_id: twilight_channel.parent_id,
            permission_overwrites: twilight_channel.permission_overwrites,
            position: twilight_channel.position,
            rate_limit_per_user: twilight_channel.rate_limit_per_user,
            recipients: twilight_channel.recipients,
            rtc_region: twilight_channel.rtc_region,
            thread_metadata: twilight_channel.thread_metadata,
            topic: twilight_channel.topic,
            user_limit: twilight_channel.user_limit,
            video_quality_mode: twilight_channel.video_quality_mode,
        }
    }
}

impl From<Channel> for LuroChannel {
    fn from(luro_channel: Channel) -> Self {
        Self {
            application_id: luro_channel.application_id,
            applied_tags: luro_channel.applied_tags,
            available_tags: luro_channel.available_tags,
            bitrate: luro_channel.bitrate,
            default_auto_archive_duration: luro_channel.default_auto_archive_duration,
            default_forum_layout: luro_channel.default_forum_layout,
            default_reaction_emoji: luro_channel.default_reaction_emoji,
            default_sort_order: luro_channel.default_sort_order,
            default_thread_rate_limit_per_user: luro_channel.default_thread_rate_limit_per_user,
            flags: luro_channel.flags,
            guild_id: luro_channel.guild_id,
            icon: luro_channel.icon,
            id: luro_channel.id,
            invitable: luro_channel.invitable,
            kind: luro_channel.kind,
            last_message_id: luro_channel.last_message_id,
            last_pin_timestamp: luro_channel.last_pin_timestamp,
            managed: luro_channel.managed,
            member: luro_channel.member,
            member_count: luro_channel.member_count,
            message_count: luro_channel.message_count,
            name: luro_channel.name,
            newly_created: luro_channel.newly_created,
            nsfw: luro_channel.nsfw,
            owner_id: luro_channel.owner_id,
            parent_id: luro_channel.parent_id,
            permission_overwrites: luro_channel.permission_overwrites,
            position: luro_channel.position,
            rate_limit_per_user: luro_channel.rate_limit_per_user,
            recipients: luro_channel.recipients,
            rtc_region: luro_channel.rtc_region,
            thread_metadata: luro_channel.thread_metadata,
            topic: luro_channel.topic,
            user_limit: luro_channel.user_limit,
            video_quality_mode: luro_channel.video_quality_mode,
            deleted: false,
        }
    }
}

impl TryFrom<DbChannel> for LuroChannel {
    type Error = anyhow::Error;

    fn try_from(db_channel: DbChannel) -> Result<Self, Self::Error> {
        Ok(Self {
            deleted: db_channel.deleted,
            application_id: db_channel.application_id.map(|x| Id::new(x as u64)),
            applied_tags: db_channel
                .applied_tags
                .map(|x| x.into_iter().map(|x| Id::new(x as u64)).collect::<Vec<_>>()),
            available_tags: db_channel.available_tags.map(|x| x.into_iter().map(|x| x.0).collect::<Vec<_>>()),
            bitrate: db_channel.bitrate.map(|x| x as u32),
            default_auto_archive_duration: db_channel
                .default_auto_archive_duration
                .map(|x| AutoArchiveDuration::from(x as u16)),
            default_forum_layout: db_channel.default_forum_layout.map(|x| ForumLayout::from(x as u8)),
            default_reaction_emoji: db_channel.default_reaction_emoji.map(|x| x.0),
            default_sort_order: db_channel.default_sort_order.map(|x| ForumSortOrder::from(x as u8)),
            default_thread_rate_limit_per_user: db_channel.default_thread_rate_limit_per_user.map(|x| x as u16),
            flags: db_channel.flags.map(|x| ChannelFlags::from_bits_retain(x as u64)),
            guild_id: db_channel.guild_id.map(|x| Id::new(x as u64)),
            icon: match db_channel.icon {
                Some(icon) => Some(ImageHash::parse(icon.as_bytes())?),
                None => None,
            },
            id: Id::new(db_channel.channel_id as u64),
            invitable: db_channel.invitable,
            kind: ChannelType::from(db_channel.kind as u8),
            last_message_id: db_channel.last_message_id.map(|x| Id::new(x as u64)),
            last_pin_timestamp: match db_channel.last_pin_timestamp {
                Some(timestamp) => Some(Timestamp::from_secs(timestamp.unix_timestamp())?),
                None => None,
            },
            managed: db_channel.managed,
            member: db_channel.member.map(|x| x.0),
            member_count: db_channel.member_count.map(|x| x as i8),
            message_count: db_channel.message_count.map(|x| x as u32),
            name: db_channel.name,
            newly_created: db_channel.newly_created,
            nsfw: db_channel.nsfw,
            owner_id: db_channel.owner_id.map(|x| Id::new(x as u64)),
            parent_id: db_channel.parent_id.map(|x| Id::new(x as u64)),
            permission_overwrites: db_channel
                .permission_overwrites
                .map(|x| x.into_iter().map(|x| x.0).collect::<Vec<_>>()),
            position: db_channel.position,
            rate_limit_per_user: db_channel.rate_limit_per_user.map(|x| x as u16),
            recipients: None, // TODO: Implement this
            rtc_region: db_channel.rtc_region,
            thread_metadata: db_channel.thread_metadata.map(|x| x.0),
            topic: db_channel.topic,
            user_limit: db_channel.user_limit.map(|x| x as u32),
            video_quality_mode: db_channel.video_quality_mode.map(|x| VideoQualityMode::from(x as u8)),
        })
    }
}

impl TryFrom<LuroChannel> for DbChannel {
    type Error = anyhow::Error;

    fn try_from(luro_channel: LuroChannel) -> Result<Self, Self::Error> {
        Ok(Self {
            application_id: luro_channel.application_id.map(|x| x.get() as i64),
            applied_tags: luro_channel
                .applied_tags
                .map(|x| x.into_iter().map(|x| x.get() as i64).collect::<Vec<_>>()),
            available_tags: luro_channel.available_tags.map(|x| x.into_iter().map(Json).collect::<Vec<_>>()),
            bitrate: luro_channel.bitrate.map(|x| x as i32),
            deleted: luro_channel.deleted,
            default_auto_archive_duration: luro_channel.default_auto_archive_duration.map(|x| x.number() as i16),
            default_forum_layout: luro_channel.default_forum_layout.map(|x| u8::from(x) as i16),
            default_reaction_emoji: luro_channel.default_reaction_emoji.map(Json),
            default_sort_order: luro_channel.default_sort_order.map(|x| u8::from(x) as i16),
            default_thread_rate_limit_per_user: luro_channel.default_thread_rate_limit_per_user.map(|x| x as i16),
            flags: luro_channel.flags.map(|x| x.bits() as i64),
            guild_id: luro_channel.guild_id.map(|x| x.get() as i64),
            icon: luro_channel.icon.map(|x| x.to_string()),
            channel_id: luro_channel.id.get() as i64,
            invitable: luro_channel.invitable,
            kind: u8::from(luro_channel.kind) as i16,
            last_message_id: luro_channel.last_message_id.map(|x| x.get() as i64),
            last_pin_timestamp: match luro_channel.last_pin_timestamp {
                Some(timestamp) => Some(OffsetDateTime::from_unix_timestamp(timestamp.as_secs())?),
                None => None,
            },
            managed: luro_channel.managed,
            member: luro_channel.member.map(Json),
            member_count: luro_channel.member_count.map(|x| x as i16),
            message_count: luro_channel.message_count.map(|x| x as i32),
            name: luro_channel.name,
            newly_created: luro_channel.newly_created,
            nsfw: luro_channel.nsfw,
            owner_id: luro_channel.owner_id.map(|x| x.get() as i64),
            parent_id: luro_channel.parent_id.map(|x| x.get() as i64),
            permission_overwrites: luro_channel
                .permission_overwrites
                .map(|x| x.into_iter().map(Json).collect::<Vec<_>>()),
            position: luro_channel.position,
            rate_limit_per_user: luro_channel.rate_limit_per_user.map(|x| x as i16),
            recipients: None, // TODO: Implement this
            rtc_region: luro_channel.rtc_region,
            thread_metadata: luro_channel.thread_metadata.map(Json),
            topic: luro_channel.topic,
            user_limit: luro_channel.user_limit.map(|x| x as i32),
            video_quality_mode: luro_channel.video_quality_mode.map(|x| u8::from(x) as i16),
        })
    }
}
