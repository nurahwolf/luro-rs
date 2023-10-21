use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, BTreeMap, HashMap},
};

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use twilight_cache_inmemory::model::CachedMember;
use twilight_model::{
    gateway::payload::incoming::{MemberAdd, MemberUpdate},
    guild::{Member, PartialMember},
    id::{
        marker::{GuildMarker, MessageMarker, UserMarker},
        Id,
    },
    user::{CurrentUser, PremiumType, User, UserFlags},
    util::ImageHash,
};

/// A [HashMap] containing user specific settings ([LuroUser]), keyed by [UserMarker].
pub type LuroUsers = HashMap<Id<UserMarker>, LuroUser>;
use crate::message::LuroMessage;

use self::{actions::UserActions, character::CharacterProfile, marriages::UserMarriages, member::LuroMember};

pub mod actions;
pub mod actions_type;
pub mod character;
pub mod marriages;
pub mod member;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
/// A warpper around [User], with [Member] details if [Id<GuildMarker>] was present on type creation.
/// Details are primarily fetched from the database, but this type can be instanced from a [User] / [Member] if that fails.
/// Also holds some additional which are relevent to Luro only. These are empty if the type was not instanced from the database.
///
/// Check [LuroUserType] to know how this type was instanced.
pub struct LuroUser {
    /// Accent color of the user's banner.
    ///
    /// This is an integer representation of a hexadecimal color code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<ImageHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_decoration: Option<ImageHash>,
    /// Hash of the user's banner image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<ImageHash>,
    #[serde(default)]
    pub bot: bool,
    #[serde(default)]
    pub discriminator: u16,
    /// User's global display name, if set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<UserFlags>,
    pub id: Id<UserMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(default)]
    pub mfa_enabled: bool,
    /// The user's raw username.
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_type: Option<PremiumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_flags: Option<UserFlags>,
    #[serde(default)]
    pub system: bool,
    #[serde(default)]
    pub verified: bool,
    /// How many words they have said
    #[serde(default)]
    pub wordcount: usize,
    /// The sum of the length of all the words they have said. This is then divided by their wordcount to work out an average.
    #[serde(default)]
    pub averagesize: usize,
    #[serde(deserialize_with = "deserialize_wordsize", serialize_with = "serialize_wordsize")]
    /// A hashmap containing the word length, and how many times it has appeared
    /// TODO: The key for this is stored as a string on the disk which is only needed for toml
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub wordsize: BTreeMap<usize, usize>,
    /// A hashmap containing a count on how often a particular word appears
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub words: BTreeMap<String, usize>,
    /// An tuple of warnings wrapped in a vec. The first value is the warning, and the second is whoever warned the person
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub warnings: Vec<(String, Id<UserMarker>)>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub messages: HashMap<Id<MessageMarker>, LuroMessage>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub moderation_actions: Vec<UserActions>,
    #[serde(default)]
    pub moderation_actions_performed: usize,
    /// A simple tally of how many times a user has fucked up and needed to edit their message.
    #[serde(default)]
    pub message_edits: usize,
    /// The user's marriages
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub marriages: BTreeMap<Id<UserMarker>, UserMarriages>,
    /// A list of member instances across guilds
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub guilds: HashMap<Id<GuildMarker>, LuroMember>,
    /// The user's character profiles
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub characters: BTreeMap<String, CharacterProfile>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub character_prefix: BTreeMap<String, String>,
    #[serde(default)]
    pub user_permissions: LuroUserPermissions,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LuroUserPermissions {
    #[default]
    User,
    Owner,
    Administrator,
}

impl Eq for LuroUser {}

impl Ord for LuroUser {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for LuroUser {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&CurrentUser> for LuroUser {
    fn from(user: &CurrentUser) -> Self {
        let mut luro = Self::new(user.id);
        luro.update_currentuser(user);
        luro
    }
}

impl From<&User> for LuroUser {
    fn from(user: &User) -> Self {
        let mut luro = Self::new(user.id);
        luro.update_user(user);
        luro
    }
}

impl LuroUser {
    pub fn new(id: Id<UserMarker>) -> Self {
        Self {
            accent_color: Default::default(),
            avatar: Default::default(),
            avatar_decoration: Default::default(),
            banner: Default::default(),
            bot: Default::default(),
            discriminator: Default::default(),
            global_name: Default::default(),
            email: Default::default(),
            flags: Default::default(),
            id,
            locale: Default::default(),
            mfa_enabled: Default::default(),
            name: Default::default(),
            premium_type: Default::default(),
            public_flags: Default::default(),
            system: Default::default(),
            verified: Default::default(),
            wordcount: Default::default(),
            averagesize: Default::default(),
            wordsize: Default::default(),
            words: Default::default(),
            warnings: Default::default(),
            messages: Default::default(),
            moderation_actions: Default::default(),
            moderation_actions_performed: Default::default(),
            message_edits: Default::default(),
            marriages: Default::default(),
            guilds: Default::default(),
            characters: Default::default(),
            character_prefix: Default::default(),
            user_permissions: Default::default(),
        }
    }

    /// Update this type from a luro user
    pub fn update_lurouser(&mut self, luro: &Self) -> &mut Self {
        self.accent_color = luro.accent_color;
        self.avatar = luro.avatar;
        self.banner = luro.banner;
        self.bot = luro.bot;
        self.discriminator = luro.discriminator;
        self.email = luro.email.clone();
        self.flags = luro.flags;
        self.id = luro.id;
        self.locale = luro.locale.clone();
        self.mfa_enabled = luro.mfa_enabled;
        self.name = luro.name.clone();
        self.premium_type = luro.premium_type;
        self.public_flags = luro.public_flags;
        self.verified = luro.verified;
        for (guild_id, member) in luro.guilds.iter() {
            match self.guilds.entry(*guild_id) {
                Entry::Vacant(entry) => {
                    entry.insert(member.clone());
                }
                Entry::Occupied(mut entry) => {
                    let entry = entry.get_mut();
                    entry.avatar = member.avatar;
                    entry.communication_disabled_until = member.communication_disabled_until;
                    entry.deaf = member.deaf;
                    entry.flags = member.flags;
                    entry.joined_at = member.joined_at;
                    entry.mute = member.mute;
                    entry.nick = member.nick.clone();
                    entry.premium_since = member.premium_since;
                    entry.role_ids = member.role_ids.clone();
                    if let Some(permissions) = member.permissions {
                        entry.permissions = Some(permissions);
                    }
                }
            };
        }
        self
    }

    /// Update this type from a currentuser.
    pub fn update_currentuser(&mut self, user: &CurrentUser) -> &mut Self {
        self.accent_color = user.accent_color;
        self.avatar = user.avatar;
        self.banner = user.banner;
        self.bot = user.bot;
        self.discriminator = user.discriminator;
        self.email = user.email.clone();
        self.flags = user.flags;
        self.id = user.id;
        self.locale = user.locale.clone();
        self.mfa_enabled = user.mfa_enabled;
        self.name = user.name.clone();
        self.premium_type = user.premium_type;
        self.public_flags = user.public_flags;
        self.verified = user.verified.unwrap_or_default();
        self
    }

    /// Update this type from a [CachedMember].
    pub fn update_cached_member(&mut self, guild_id: &Id<GuildMarker>, member: &CachedMember) -> &mut Self {
        self.guilds
            .entry(*guild_id)
            .and_modify(|e| {
                e.avatar = member.avatar();
                e.communication_disabled_until = member.communication_disabled_until();
                e.deaf = member.deaf().unwrap_or_default();
                e.flags = member.flags();
                e.id = Some(member.user_id());
                e.joined_at = member.joined_at();
                e.mute = member.mute().unwrap_or_default();
                e.flags = member.flags();
                e.nick = member.nick().map(|s| s.to_string());
                e.pending = member.pending();
                e.premium_since = member.premium_since();
                e.role_ids = member.roles().to_vec();
            })
            .or_insert(LuroMember::from(member));
        self
    }

    /// Update this type from a user.
    pub fn update_user(&mut self, user: &User) -> &mut Self {
        self.accent_color = user.accent_color;
        self.avatar = user.avatar;
        self.avatar_decoration = user.avatar_decoration;
        self.banner = user.banner;
        self.bot = user.bot;
        self.discriminator = user.discriminator;
        self.email = user.email.clone();
        self.flags = user.flags;
        self.global_name = user.global_name.clone();
        self.id = user.id;
        self.locale = user.locale.clone();
        self.mfa_enabled = user.mfa_enabled.unwrap_or_default();
        self.name = user.name.clone();
        self.premium_type = user.premium_type;
        self.public_flags = user.public_flags;
        self.system = user.system.unwrap_or_default();
        self.verified = user.verified.unwrap_or_default();
        self
    }

    pub fn update_member_add(&mut self, event: Box<MemberAdd>) -> &mut Self {
        self.update_user(&event.user);
        match self.guilds.entry(event.guild_id) {
            Entry::Vacant(entry) => entry.insert(LuroMember::from(event)),
            Entry::Occupied(mut entry) => entry.get_mut().update_member_add(event),
        };
        self
    }

    pub fn update_member_update(&mut self, event: Box<MemberUpdate>) -> &mut Self {
        self.update_user(&event.user);
        match self.guilds.entry(event.guild_id) {
            Entry::Vacant(entry) => entry.insert(LuroMember::from(event)),
            Entry::Occupied(mut entry) => entry.get_mut().update_member_update(event),
        };
        self
    }

    /// Update this type from a member. Consider creating a default and then calling this function if you need a blank slate
    pub fn update_member(&mut self, guild_id: &Id<GuildMarker>, member: &Member) -> &mut Self {
        self.update_user(&member.user);
        match self.guilds.entry(*guild_id) {
            Entry::Vacant(entry) => entry.insert(LuroMember::from(member)),
            Entry::Occupied(mut entry) => entry.get_mut().update_member(member),
        };
        self
    }

    /// Update this type from a partial member. Consider creating a default and then calling this function if you need a blank slate
    pub fn update_partialmember(&mut self, guild_id: &Id<GuildMarker>, member: &PartialMember) -> &mut Self {
        if let Some(ref user) = member.user {
            self.update_user(user);
        }
        match self.guilds.entry(*guild_id) {
            Entry::Vacant(entry) => entry.insert(LuroMember::from(member)),
            Entry::Occupied(mut entry) => entry.get_mut().update_partialmember(member),
        };
        self
    }

    /// Return a string that is a link to the user's guild avatar if set, defaulting to the user
    pub fn guild_avatar(&self, guild_id: &Id<GuildMarker>) -> String {
        let guild = match self.guilds.get(guild_id) {
            Some(guild) => guild,
            None => return self.avatar(),
        };

        match guild.avatar {
            Some(avatar) => match avatar.is_animated() {
                true => format!(
                    "https://cdn.discordapp.com/guilds/{guild_id}/users/{}/avatars/{avatar}.gif?size=2048",
                    self.id
                ),
                false => format!(
                    "https://cdn.discordapp.com/guilds/{guild_id}/users/{}/avatars/{avatar}.png?size=2048",
                    self.id
                ),
            },
            None => self.avatar(),
        }
    }

    /// Return a string that is a link to the user's avatar
    pub fn avatar(&self) -> String {
        let user_id = self.id;
        match self.avatar {
            Some(avatar) => match avatar.is_animated() {
                true => format!("https://cdn.discordapp.com/avatars/{user_id}/{avatar}.gif?size=2048"),
                false => format!("https://cdn.discordapp.com/avatars/{user_id}/{avatar}.png?size=2048"),
            },
            None => format!("https://cdn.discordapp.com/embed/avatars/{}.png?size=2048", self.id.get() > 22 % 6),
        }
    }

    /// Return a string that is a link to the user's banner, or [None] if they don't have one
    pub fn banner(&self) -> Option<String> {
        self.banner.map(|banner| match banner.is_animated() {
            true => format!("https://cdn.discordapp.com/banners/{}/{banner}.gif?size=4096", self.id),
            false => format!("https://cdn.discordapp.com/banners/{}/{banner}.png?size=4096", self.id),
        })
    }

    /// Get's the member's nickname if set, otherwise gets the user's name
    ///
    /// Returns the first match
    /// Member Nickname -> Global Name -> Username -> Legacy Username
    pub fn member_name(&self, guild_id: &Option<Id<GuildMarker>>) -> String {
        let guild_id = match guild_id {
            Some(guild_id) => guild_id,
            None => return self.name(),
        };

        let member = match self.guilds.get(guild_id) {
            Some(guild) => guild,
            None => return self.name(),
        };

        match &member.nick {
            Some(nickname) => nickname.clone(),
            None => self.name(),
        }
    }

    /// Get's the user's pretty name
    ///
    /// Returns the first match
    /// Global Name -> Username -> Legacy Username
    pub fn name(&self) -> String {
        match &self.global_name {
            Some(global_name) => global_name.clone(),
            None => self.username(),
        }
    }

    /// Get's the user's username name
    ///
    /// Returns the first match
    /// Username -> Legacy Username
    pub fn username(&self) -> String {
        match self.discriminator == 0 {
            true => self.name.clone(),
            false => format!("{}#{}", self.name, self.discriminator),
        }
    }
}

pub fn serialize_wordsize<S>(input: &BTreeMap<usize, usize>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let data = input
        .iter()
        .map(|(str_key, value)| (str_key.to_string(), *value))
        .collect::<BTreeMap<String, usize>>();

    s.collect_map(data)
}

pub fn deserialize_wordsize<'de, D>(deserializer: D) -> Result<BTreeMap<usize, usize>, D::Error>
where
    D: Deserializer<'de>,
{
    let str_map = BTreeMap::<String, usize>::deserialize(deserializer)?;
    let original_len = str_map.len();
    let data = {
        str_map
            .into_iter()
            .map(|(str_key, value)| match str_key.parse() {
                Ok(int_key) => Ok((int_key, value)),
                Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(&str_key), &"a non-negative integer")),
            })
            .collect::<Result<BTreeMap<_, _>, _>>()?
    };
    // multiple strings could parse to the same int, e.g "0" and "00"
    if data.len() < original_len {
        return Err(de::Error::custom("detected duplicate integer key"));
    }
    Ok(data)
}
