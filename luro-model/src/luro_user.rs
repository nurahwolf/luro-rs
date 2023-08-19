use std::collections::{btree_map::Entry, BTreeMap};

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use twilight_model::{
    guild::{Member, PartialMember},
    id::{
        marker::{GuildMarker, MessageMarker, UserMarker},
        Id
    },
    user::{CurrentUser, PremiumType, User, UserFlags},
    util::ImageHash
};

use crate::{luro_member::LuroMember, luro_message::LuroMessage, user_actions::UserActions, user_marriages::UserMarriages, character_profile::CharacterProfile};

/// Some nice functionality primarily around [User] and [Member], with some added goodness
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
    #[serde(default)]
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(default)]
    pub mfa_enabled: bool,
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
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub messages: BTreeMap<Id<MessageMarker>, LuroMessage>,
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
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub guilds: BTreeMap<Id<GuildMarker>, LuroMember>,
    /// The user's character profiles
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub characters: BTreeMap<String, CharacterProfile>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub character_prefix: BTreeMap<String, String>,
}

impl From<&CurrentUser> for LuroUser {
    fn from(user: &CurrentUser) -> Self {
        let mut luro = Self::default();
        luro.update_currentuser(user);
        luro
    }
}

impl From<&User> for LuroUser {
    fn from(user: &User) -> Self {
        let mut luro = Self::default();
        luro.update_user(user);
        luro
    }
}

impl LuroUser {
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
        for (guild_id, member) in &luro.guilds {
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
        self.id = user.id.get();
        self.locale = user.locale.clone();
        self.mfa_enabled = user.mfa_enabled;
        self.name = user.name.clone();
        self.premium_type = user.premium_type;
        self.public_flags = user.public_flags;
        self.verified = user.verified.unwrap_or_default();
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
        self.id = user.id.get();
        self.locale = user.locale.clone();
        self.mfa_enabled = user.mfa_enabled.unwrap_or_default();
        self.name = user.name.clone();
        self.premium_type = user.premium_type;
        self.public_flags = user.public_flags;
        self.system = user.system.unwrap_or_default();
        self.verified = user.verified.unwrap_or_default();
        self
    }

    /// Update this type from a member. Consider creating a default and then calling this function if you need a blank slate
    pub fn update_member(&mut self, guild_id: &Id<GuildMarker>, member: &Member) -> &mut Self {
        self.update_user(&member.user);
        match self.guilds.entry(*guild_id) {
            Entry::Vacant(entry) => entry.insert(LuroMember::from(member)),
            Entry::Occupied(mut entry) => entry.get_mut().update_member(member)
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
            Entry::Occupied(mut entry) => entry.get_mut().update_partialmember(member)
        };
        self
    }

    /// Return a string that is a link to the user's guild avatar if set, defaulting to the user
    pub fn guild_avatar(&self, guild_id: &Id<GuildMarker>) -> String {
        let guild = match self.guilds.get(guild_id) {
            Some(guild) => guild,
            None => return self.avatar()
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
                )
            },
            None => self.avatar()
        }
    }

    /// Return a string that is a link to the user's avatar
    pub fn avatar(&self) -> String {
        match self.avatar {
            Some(avatar) => match avatar.is_animated() {
                true => format!("https://cdn.discordapp.com/avatars/{}/{avatar}.gif?size=2048", self.id),
                false => format!("https://cdn.discordapp.com/avatars/{}/{avatar}.png?size=2048", self.id)
            },
            None => format!(
                "https://cdn.discordapp.com/embed/avatars/{}.png?size=2048",
                self.discriminator % 5
            )
        }
    }

    /// Return a string that is a link to the user's banner, or [None] if they don't have one
    pub fn banner(&self) -> Option<String> {
        self.banner.map(|banner| match banner.is_animated() {
            true => format!("https://cdn.discordapp.com/banners/{}/{banner}.gif?size=4096", self.id),
            false => format!("https://cdn.discordapp.com/banners/{}/{banner}.png?size=4096", self.id)
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

        let guild = match self.guilds.get(guild_id) {
            Some(guild) => guild,
            None => return self.name()
        };

        match &guild.nick {
            Some(nickname) => nickname.clone(),
            None => self.name()
        }
    }

    /// Get's the user's pretty name
    ///
    /// Returns the first match
    /// Global Name -> Username -> Legacy Username
    pub fn name(&self) -> String {
        match &self.global_name {
            Some(global_name) => global_name.clone(),
            None => match self.discriminator == 0 {
                true => self.name.clone(),
                false => format!("{}#{}", self.name, self.discriminator)
            }
        }
    }
}

pub fn serialize_wordsize<S>(input: &BTreeMap<usize, usize>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    let data = input
        .iter()
        .map(|(str_key, value)| (str_key.to_string(), *value))
        .collect::<BTreeMap<String, usize>>();

    s.collect_map(data)
}

pub fn deserialize_wordsize<'de, D>(deserializer: D) -> Result<BTreeMap<usize, usize>, D::Error>
where
    D: Deserializer<'de>
{
    let str_map = BTreeMap::<String, usize>::deserialize(deserializer)?;
    let original_len = str_map.len();
    let data = {
        str_map
            .into_iter()
            .map(|(str_key, value)| match str_key.parse() {
                Ok(int_key) => Ok((int_key, value)),
                Err(_) => Err(de::Error::invalid_value(
                    de::Unexpected::Str(&str_key),
                    &"a non-negative integer"
                ))
            })
            .collect::<Result<BTreeMap<_, _>, _>>()?
    };
    // multiple strings could parse to the same int, e.g "0" and "00"
    if data.len() < original_len {
        return Err(de::Error::custom("detected duplicate integer key"));
    }
    Ok(data)
}
