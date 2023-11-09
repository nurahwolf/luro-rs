use serde::{Deserialize, Serialize};
use twilight_model::{
    channel::{
        forum::{DefaultReaction, ForumLayout, ForumSortOrder, ForumTag},
        permission_overwrite::PermissionOverwrite,
        thread::{AutoArchiveDuration, ThreadMember, ThreadMetadata},
        ChannelFlags, ChannelType, VideoQualityMode,
    },
    id::{
        marker::{ApplicationMarker, ChannelMarker, GenericMarker, GuildMarker, TagMarker, UserMarker},
        Id,
    },
    user::User,
    util::{ImageHash, Timestamp},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
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

impl From<twilight_model::channel::Channel> for Channel {
    fn from(twilight_channel: twilight_model::channel::Channel) -> Self {
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
            deleted: false,
        }
    }
}

impl From<Channel> for twilight_model::channel::Channel {
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
        }
    }
}