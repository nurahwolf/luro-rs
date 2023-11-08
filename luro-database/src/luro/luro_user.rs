use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use twilight_model::{
    guild::Member,
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
    user::{PremiumType, User, UserFlags},
    util::ImageHash,
};

use crate::{LuroMember, LuroUserData, LuroUserType};

mod fetch_character;
mod fetch_characters;
mod fetch_marriages;
mod fetch_message_count;
mod sync;
mod update_character;
mod update_character_prefix;
mod update_character_text;
mod update_permissions;

/// A warpper around [User], with [Member] details if [Id<GuildMarker>] was present on type creation.
/// Details are primarily fetched from the database, but this type can be instanced from a [User] / [Member] if that fails.
/// Also holds some additional which are relevent to Luro only. These are empty if the type was not instanced from the database.
///
/// Check [LuroUserType] to know how this type was instanced.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LuroUser {
    pub accent_colour: Option<u32>,
    pub avatar_decoration: Option<ImageHash>,
    pub avatar: Option<ImageHash>,
    pub banner: Option<ImageHash>,
    pub bot: bool,
    pub data: Option<LuroUserData>,
    pub discriminator: u16,
    pub email: Option<String>,
    pub flags: Option<UserFlags>,
    pub global_name: Option<String>,
    pub instance: LuroUserType,
    pub locale: Option<String>,
    pub member: Option<LuroMember>,
    pub mfa_enabled: Option<bool>,
    pub name: String,
    pub premium_type: Option<PremiumType>,
    pub public_flags: Option<UserFlags>,
    pub system: Option<bool>,
    pub user_id: Id<UserMarker>,
    pub verified: Option<bool>,
}

impl From<User> for LuroUser {
    fn from(user: User) -> Self {
        Self {
            instance: LuroUserType::User,
            member: None,
            data: None,
            accent_colour: user.accent_color,
            avatar_decoration: user.avatar_decoration,
            avatar: user.avatar,
            banner: user.banner,
            bot: user.bot,
            discriminator: user.discriminator,
            email: user.email,
            flags: user.flags,
            global_name: user.global_name,
            locale: user.locale,
            mfa_enabled: user.mfa_enabled,
            name: user.name,
            premium_type: user.premium_type,
            public_flags: user.public_flags,
            system: user.system,
            verified: user.verified,
            user_id: user.id,
        }
    }
}

impl From<(Member, Id<GuildMarker>)> for LuroUser {
    fn from((member, guild_id): (Member, Id<GuildMarker>)) -> Self {
        Self {
            instance: LuroUserType::Member,
            data: None,
            member: Some(LuroMember {
                data: None,
                avatar: member.avatar,
                boosting_since: member.premium_since,
                communication_disabled_until: member.communication_disabled_until,
                joined_at: member.joined_at,
                deafened: member.deaf,
                flags: member.flags,
                guild_id,
                muted: member.mute,
                nickname: member.nick,
                pending: member.pending,
                roles: member.roles,
                user_id: member.user.id,
            }),
            accent_colour: member.user.accent_color,
            avatar_decoration: member.user.avatar_decoration,
            avatar: member.user.avatar,
            banner: member.user.banner,
            bot: member.user.bot,
            discriminator: member.user.discriminator,
            email: member.user.email,
            flags: member.user.flags,
            global_name: member.user.global_name,
            locale: member.user.locale,
            mfa_enabled: member.user.mfa_enabled,
            name: member.user.name.clone(),
            premium_type: member.user.premium_type,
            public_flags: member.user.public_flags,
            system: member.user.system,
            verified: member.user.verified,
            user_id: member.user.id,
        }
    }
}

impl LuroUser {
    /// Update the contained member.
    pub fn update_member(&mut self, member: impl Into<LuroMember>) -> &mut Self {
        let member = member.into();
        self.member = Some(member);
        self
    }

    /// Return a string that is a link to the user's avatar
    pub fn avatar_url(&self) -> String {
        let user_id = self.user_id;
        if let Some(member) = &self.member {
            let guild_id = member.guild_id;

            if let Some(avatar) = member.avatar {
                return match avatar.is_animated() {
                    true => format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{avatar}.gif?size=2048"),
                    false => format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{avatar}.png?size=2048"),
                };
            }
        }

        match self.avatar {
            Some(avatar) => match avatar.is_animated() {
                true => format!("https://cdn.discordapp.com/avatars/{user_id}/{avatar}.gif?size=2048"),
                false => format!("https://cdn.discordapp.com/avatars/{user_id}/{avatar}.png?size=2048"),
            },
            _ => format!("https://cdn.discordapp.com/avatars/{}.png?size=2048", self.user_id.get() > 22 % 6),
        }
    }

    /// Return a string that is a link to the user's banner, or [None] if they don't have one
    pub fn banner_url(&self) -> Option<String> {
        self.banner.map(|x| match x.is_animated() {
            true => format!("https://cdn.discordapp.com/banners/{}/{}.gif?size=4096", self.user_id, x),
            false => format!("https://cdn.discordapp.com/banners/{}/{}.png?size=4096", self.user_id, x),
        })
    }

    /// Get's the user's preferred / pretty name
    ///
    /// Returns the first match
    /// Member Nickname -> Global Name -> Username -> Legacy Username
    pub fn name(&self) -> String {
        if let Some(Some(nickname)) = self.member.as_ref().map(|x| x.nickname.clone()) {
            return nickname;
        }

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

impl From<LuroUser> for User {
    fn from(luro_user: LuroUser) -> Self {
        Self {
            accent_color: luro_user.accent_colour,
            bot: luro_user.bot,
            discriminator: luro_user.discriminator,
            email: luro_user.email,
            flags: luro_user.flags,
            global_name: luro_user.global_name,
            locale: luro_user.locale,
            mfa_enabled: luro_user.mfa_enabled,
            name: luro_user.name,
            premium_type: luro_user.premium_type,
            public_flags: luro_user.public_flags,
            system: luro_user.system,
            verified: luro_user.verified,
            id: luro_user.user_id,
            avatar: luro_user.avatar,
            avatar_decoration: luro_user.avatar_decoration,
            banner: luro_user.banner,
        }
    }
}

impl TryFrom<LuroUser> for Member {
    type Error = anyhow::Error;

    fn try_from(luro_user: LuroUser) -> Result<Self, Self::Error> {
        match luro_user.member {
            Some(ref member) => Ok(Self {
                avatar: member.avatar,
                communication_disabled_until: member.communication_disabled_until,
                deaf: member.deafened,
                flags: member.flags,
                joined_at: member.joined_at,
                mute: member.muted,
                nick: member.nickname.clone(),
                pending: member.pending,
                premium_since: member.boosting_since,
                roles: member.roles.clone(),
                user: luro_user.into(),
            }),
            None => Err(anyhow!(
                "Luro User was not instanced from a type containing member data: '{}'",
                luro_user.instance
            )),
        }
    }
}
