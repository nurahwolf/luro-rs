use sqlx::types::Json;
use time::OffsetDateTime;
use twilight_model::{
    channel::{
        forum::{DefaultReaction, ForumTag},
        permission_overwrite::PermissionOverwrite,
        thread::{ThreadMember, ThreadMetadata},
        Channel,
    },
    gateway::payload::incoming::{ChannelCreate, ChannelDelete, ChannelPinsUpdate, ChannelUpdate},
};

mod count_channels;
mod update_channel;

pub struct DbChannel {
    pub application_id: Option<i64>, // applications
    pub channel_id: i64,             // channels
    pub deleted: bool,
    pub guild_id: Option<i64>, // guilds
    pub applied_tags: Option<Vec<i64>>,
    pub available_tags: Option<Vec<Json<ForumTag>>>,
    pub bitrate: Option<i32>,
    pub default_auto_archive_duration: Option<i16>,
    pub default_forum_layout: Option<i16>,
    pub default_reaction_emoji: Option<Json<DefaultReaction>>,
    pub default_sort_order: Option<i16>,
    pub default_thread_rate_limit_per_user: Option<i16>,
    pub flags: Option<i64>,
    pub icon: Option<String>,
    pub invitable: Option<bool>,
    pub kind: i16,
    pub last_message_id: Option<i64>,
    pub last_pin_timestamp: Option<OffsetDateTime>,
    pub managed: Option<bool>,
    pub member: Option<Json<ThreadMember>>,
    pub member_count: Option<i16>,
    pub message_count: Option<i32>,
    pub name: Option<String>,
    pub newly_created: Option<bool>,
    pub nsfw: Option<bool>,
    pub owner_id: Option<i64>,  // guild_members
    pub parent_id: Option<i64>, // guild_channels
    pub permission_overwrites: Option<Vec<Json<PermissionOverwrite>>>,
    pub position: Option<i32>,
    pub rate_limit_per_user: Option<i16>,
    pub recipients: Option<Vec<i64>>, // users
    pub rtc_region: Option<String>,
    pub thread_metadata: Option<Json<ThreadMetadata>>,
    pub topic: Option<String>,
    pub user_limit: Option<i32>,
    pub video_quality_mode: Option<i16>,
}

pub enum DbChannelType {
    DbChannel(DbChannel),
    Channel(Channel),
    ChannelCreate(Box<ChannelCreate>),
    ChannelDelete(Box<ChannelDelete>),
    ChannelUpdate(Box<ChannelUpdate>),
    ChannelPinsUpdate(ChannelPinsUpdate),
}
