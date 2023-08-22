#[cfg(feature = "toml-driver")]
use crate::database::drivers::toml::{
    deserialize_heck::deserialize_heck, deserialize_role_positions::deserialize_role_positions, serialize_heck::serialize_heck,
    serialize_role_positions::serialize_role_positions
};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use tracing::debug;
use twilight_http::Client;
use twilight_model::{
    application::command::Command,
    channel::{message::Sticker, Channel, StageInstance},
    gateway::presence::Presence,
    guild::{
        AfkTimeout, DefaultMessageNotificationLevel, Emoji, ExplicitContentFilter, Guild, GuildFeature, MfaLevel, NSFWLevel,
        Permissions, PremiumTier, SystemChannelFlags, VerificationLevel
    },
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, RoleMarker, UserMarker},
        Id
    },
    util::{ImageHash, Timestamp},
    voice::VoiceState
};
use twilight_util::permission_calculator::PermissionCalculator;

pub mod log_channel;

use crate::{
    heck::Hecks,
    role::{LuroRole, LuroRolePositions, LuroRoles},
    user::LuroUser,
    PRIMARY_BOT_OWNER
};

/// A [HashMap] containing guild specific settings ([LuroGuild]), keyed by [GuildMarker].
pub type LuroGuilds = HashMap<Id<GuildMarker>, LuroGuild>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LuroGuild {
    /// If the guild has a custom accent set by a member of staff
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accent_colour_custom: Option<u32>,
    /// The accent colour of the guild. This is calculated by the first role in a guild that has a colour
    #[serde(default)]
    pub accent_colour: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub afk_channel_id: Option<Id<ChannelMarker>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub afk_timeout: Option<AfkTimeout>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Id<ApplicationMarker>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approximate_member_count: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approximate_presence_count: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assignable_role_blacklist: Vec<Id<RoleMarker>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner: Option<ImageHash>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catchall_log_channel: Option<Id<ChannelMarker>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub channels: Vec<Channel>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub commands: Vec<Command>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_message_notifications: Option<DefaultMessageNotificationLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discovery_splash: Option<ImageHash>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emojis: Vec<Emoji>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explicit_content_filter: Option<ExplicitContentFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub everyone_role: Option<LuroRole>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<GuildFeature>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<ImageHash>,
    #[serde(default = "id")]
    pub id: Id<GuildMarker>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub joined_at: Option<Timestamp>,
    #[serde(default)]
    pub large: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_members: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_presences: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_video_channel_users: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<Id<UserMarker>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_events_log_channel: Option<Id<ChannelMarker>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa_level: Option<MfaLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub moderator_actions_log_channel: Option<Id<ChannelMarker>>,
    #[serde(default)]
    pub name: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(
        feature = "toml-driver",
        serde(deserialize_with = "deserialize_heck", serialize_with = "serialize_heck")
    )]
    pub nsfw_hecks: Hecks,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsfw_level: Option<NSFWLevel>,
    #[serde(default = "owner_id")]
    pub owner_id: Id<UserMarker>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(default)]
    pub preferred_locale: String,
    #[serde(default)]
    pub premium_progress_bar_enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_subscription_count: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_tier: Option<PremiumTier>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub presences: Vec<Presence>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_updates_channel_id: Option<Id<ChannelMarker>>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub roles: LuroRoles,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(
        feature = "toml-driver",
        serde(
            deserialize_with = "deserialize_role_positions",
            serialize_with = "serialize_role_positions"
        )
    )]
    pub role_positions: LuroRolePositions,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules_channel_id: Option<Id<ChannelMarker>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub safety_alerts_channel_id: Option<Id<ChannelMarker>>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(
        feature = "toml-driver",
        serde(deserialize_with = "deserialize_heck", serialize_with = "serialize_heck")
    )]
    pub sfw_hecks: Hecks,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub splash: Option<ImageHash>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stage_instances: Vec<StageInstance>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stickers: Vec<Sticker>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_channel_flags: Option<SystemChannelFlags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_channel_id: Option<Id<ChannelMarker>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_events_log_channel: Option<Id<ChannelMarker>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub threads: Vec<Channel>,
    #[serde(default)]
    pub unavailable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vanity_url_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_level: Option<VerificationLevel>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub voice_states: Vec<VoiceState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub widget_channel_id: Option<Id<ChannelMarker>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub widget_enabled: Option<bool>
}

impl LuroGuild {
    pub fn sort_roles(&mut self) -> &mut Self {
        let mut roles: Vec<_> = self.roles.values().collect();

        self.role_positions.clear();
        roles.sort();

        for (iter, role) in roles.into_iter().enumerate() {
            self.role_positions.insert(iter, role.id);
        }

        self
    }

    pub fn highest_role_colour(&self) -> u32 {
        let mut colour = 0;
        for (position, id) in &self.role_positions {
            if let Some(role) = self.roles.get(id) {
                colour = role.colour;
            }

            if colour != 0 {
                // Break because we know we have a valid colour!
                debug!("Found colour at position {position}");
                break;
            }
        }
        colour
    }

    /// Return a list of a user's roles
    ///
    /// This generally should not be needed, as the user already has a list of their own roles.
    /// However, this function is useful for if you wish to get a fresh set of roles.
    pub fn user_roles(&self, user: &LuroUser) -> Vec<&LuroRole> {
        let mut user_roles = vec![];
        let user_guild = user.guilds.get(&self.id);

        if let Some(user_guild) = user_guild {
            for user_role in &user_guild.role_ids {
                for (guild_role_id, guild_role) in &self.roles {
                    if user_role == guild_role_id {
                        user_roles.push(guild_role)
                    }
                }
            }
        }

        user_roles.sort();
        user_roles
    }

    /// Gets a position in [RolePosition] for what the user's highest role is.
    ///
    /// Returns None if the user has no roles.
    pub fn user_highest_role(&self, user: &LuroUser) -> Option<(usize, Id<RoleMarker>)> {
        let user_roles = self.user_roles(user);

        match user_roles.first() {
            Some(first_role) => {
                for (role_position, role_id) in &self.role_positions {
                    if &first_role.id == role_id {
                        return Some((*role_position, *role_id));
                    }
                }
                None
            }
            None => None
        }
    }

    pub fn apply_everyone_role(&mut self, role: Option<LuroRole>) -> &mut Self {
        self.everyone_role = role;
        self
    }

    /// Returns the permissions a user may have
    pub fn user_permission(&mut self, user: &LuroUser) -> anyhow::Result<Permissions> {
        let user_permissions = &self.user_role_permissions(user);
        Ok(self.permission_calculator(user, user_permissions)?.root())
    }

    pub fn is_owner(&self, user: &LuroUser) -> bool {
        user.id == self.owner_id
    }

    pub fn permission_calculator<'a>(
        &'a self,
        user: &'a LuroUser,
        user_permissions: &'a [(Id<RoleMarker>, Permissions)]
    ) -> anyhow::Result<PermissionCalculator> {
        let everyone_role = match &self.everyone_role {
            Some(role) => role,
            None => return Err(anyhow!("Guild roles do not contain the everyone role!"))
        };
        Ok(PermissionCalculator::new(self.id, user.id, everyone_role.permissions, user_permissions).owner_id(self.owner_id))
    }

    fn user_role_permissions(&mut self, user: &LuroUser) -> Vec<(Id<RoleMarker>, Permissions)> {
        let mut roles: Vec<LuroRole> = self.user_roles(user).into_iter().cloned().collect();
        if let Some(everyone_role) = roles.iter().position(|x| x.id == self.id.cast()) {
            // let everyone_role = roles.swap_remove(everyone_role)
            self.apply_everyone_role(Some(roles.swap_remove(everyone_role)));
        }

        roles.into_iter().map(|role| role.role_permission()).collect()
    }

    /// Use Twilight's [Client] to update a guild automatically
    pub async fn update_guild_automatically(&mut self, client: &Client) -> anyhow::Result<&mut Self> {
        let guild = client.guild(self.id).await?.model().await?;
        Ok(self.update_guild(guild))
    }

    /// Update a [LuroGuild] with settings from a twilight [Guild]
    pub fn update_guild(&mut self, guild: Guild) -> &mut Self {
        let mut members = vec![];
        let mut roles = HashMap::new();

        for role in guild.roles {
            roles.insert(role.id, role.into());
        }

        self.roles = roles;
        self.sort_roles();

        for member in guild.members {
            members.push(member.user.id)
        }

        self.accent_colour = self.highest_role_colour();
        self.afk_channel_id = guild.afk_channel_id;
        self.afk_timeout = Some(guild.afk_timeout);
        self.application_id = guild.application_id;
        self.approximate_member_count = guild.approximate_member_count;
        self.approximate_presence_count = guild.approximate_presence_count;
        self.banner = guild.banner;
        self.channels = guild.channels;
        self.default_message_notifications = Some(guild.default_message_notifications);
        self.description = guild.description;
        self.discovery_splash = guild.discovery_splash;
        self.emojis = guild.emojis;
        self.explicit_content_filter = Some(guild.explicit_content_filter);
        self.features = guild.features;
        self.icon = guild.icon;
        self.id = guild.id;
        self.joined_at = guild.joined_at;
        self.large = guild.large;
        self.max_members = guild.max_members;
        self.max_presences = guild.max_presences;
        self.max_video_channel_users = guild.max_video_channel_users;
        self.member_count = guild.member_count;
        self.members = members;
        self.mfa_level = Some(guild.mfa_level);
        self.name = guild.name;
        self.owner_id = guild.owner_id;
        self.owner = guild.owner;
        self.permissions = guild.permissions;
        self.preferred_locale = guild.preferred_locale;
        self.premium_progress_bar_enabled = guild.premium_progress_bar_enabled;
        self.premium_subscription_count = guild.premium_subscription_count;
        self.premium_tier = Some(guild.premium_tier);
        self.presences = guild.presences;
        self.public_updates_channel_id = guild.afk_channel_id;
        self.rules_channel_id = guild.afk_channel_id;
        self.safety_alerts_channel_id = guild.afk_channel_id;
        self.splash = guild.splash;
        self.stage_instances = guild.stage_instances;
        self.stickers = guild.stickers;
        self.system_channel_flags = Some(guild.system_channel_flags);
        self.system_channel_id = guild.afk_channel_id;
        self.thread_events_log_channel = guild.afk_channel_id;
        self.threads = guild.threads;
        self.unavailable = guild.unavailable;
        self.vanity_url_code = guild.vanity_url_code;
        self.verification_level = Some(guild.verification_level);
        self.voice_states = guild.voice_states;
        self.widget_channel_id = guild.afk_channel_id;
        self.widget_enabled = guild.widget_enabled;
        self
    }

    pub fn new(id: Id<GuildMarker>) -> Self {
        Self {
            accent_colour_custom: Default::default(),
            accent_colour: Default::default(),
            afk_channel_id: Default::default(),
            afk_timeout: Default::default(),
            application_id: Default::default(),
            approximate_member_count: Default::default(),
            approximate_presence_count: Default::default(),
            assignable_role_blacklist: Default::default(),
            banner: Default::default(),
            catchall_log_channel: Default::default(),
            channels: Default::default(),
            commands: Default::default(),
            default_message_notifications: Default::default(),
            description: Default::default(),
            discovery_splash: Default::default(),
            emojis: Default::default(),
            explicit_content_filter: Default::default(),
            features: Default::default(),
            icon: Default::default(),
            id,
            joined_at: Default::default(),
            large: Default::default(),
            max_members: Default::default(),
            max_presences: Default::default(),
            max_video_channel_users: Default::default(),
            member_count: Default::default(),
            members: Default::default(),
            message_events_log_channel: Default::default(),
            mfa_level: Default::default(),
            moderator_actions_log_channel: Default::default(),
            name: Default::default(),
            nsfw_hecks: Default::default(),
            nsfw_level: Default::default(),
            owner_id: owner_id(),
            owner: Default::default(),
            permissions: Default::default(),
            preferred_locale: Default::default(),
            premium_progress_bar_enabled: Default::default(),
            premium_subscription_count: Default::default(),
            premium_tier: Default::default(),
            presences: Default::default(),
            public_updates_channel_id: Default::default(),
            roles: Default::default(),
            role_positions: Default::default(),
            rules_channel_id: Default::default(),
            safety_alerts_channel_id: Default::default(),
            sfw_hecks: Default::default(),
            splash: Default::default(),
            stage_instances: Default::default(),
            stickers: Default::default(),
            system_channel_flags: Default::default(),
            system_channel_id: Default::default(),
            thread_events_log_channel: Default::default(),
            threads: Default::default(),
            unavailable: Default::default(),
            vanity_url_code: Default::default(),
            verification_level: Default::default(),
            voice_states: Default::default(),
            widget_channel_id: Default::default(),
            widget_enabled: Default::default(),
            everyone_role: Default::default()
        }
    }
}

impl From<Guild> for LuroGuild {
    fn from(guild: Guild) -> Self {
        let mut luro = Self::default();
        luro.update_guild(guild);
        luro
    }
}

impl Default for LuroGuild {
    fn default() -> Self {
        Self {
            everyone_role: Default::default(),
            accent_colour_custom: Default::default(),
            accent_colour: Default::default(),
            afk_channel_id: Default::default(),
            afk_timeout: None,
            application_id: Default::default(),
            approximate_member_count: Default::default(),
            approximate_presence_count: Default::default(),
            assignable_role_blacklist: Default::default(),
            banner: Default::default(),
            catchall_log_channel: Default::default(),
            channels: Default::default(),
            commands: Default::default(),
            default_message_notifications: None,
            description: Default::default(),
            discovery_splash: Default::default(),
            emojis: Default::default(),
            explicit_content_filter: None,
            features: Default::default(),
            icon: Default::default(),
            id: Id::new(69),
            joined_at: Default::default(),
            large: Default::default(),
            max_members: Default::default(),
            max_presences: Default::default(),
            max_video_channel_users: Default::default(),
            member_count: Default::default(),
            members: Default::default(),
            message_events_log_channel: Default::default(),
            mfa_level: Some(MfaLevel::None),
            moderator_actions_log_channel: Default::default(),
            name: Default::default(),
            nsfw_hecks: Default::default(),
            nsfw_level: Default::default(),
            owner_id: PRIMARY_BOT_OWNER,
            owner: Default::default(),
            permissions: Default::default(),
            preferred_locale: Default::default(),
            premium_progress_bar_enabled: Default::default(),
            premium_subscription_count: Default::default(),
            premium_tier: Default::default(),
            presences: Default::default(),
            public_updates_channel_id: Default::default(),
            roles: Default::default(),
            rules_channel_id: Default::default(),
            safety_alerts_channel_id: Default::default(),
            sfw_hecks: Default::default(),
            splash: Default::default(),
            stage_instances: Default::default(),
            stickers: Default::default(),
            system_channel_flags: None,
            system_channel_id: Default::default(),
            thread_events_log_channel: Default::default(),
            threads: Default::default(),
            unavailable: Default::default(),
            vanity_url_code: Default::default(),
            verification_level: None,
            voice_states: Default::default(),
            widget_channel_id: Default::default(),
            widget_enabled: Default::default(),
            role_positions: Default::default()
        }
    }
}

fn id() -> Id<GuildMarker> {
    Id::new(69)
}

fn owner_id() -> Id<UserMarker> {
    PRIMARY_BOT_OWNER
}
