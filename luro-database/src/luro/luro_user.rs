use std::sync::Arc;

use anyhow::anyhow;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use tracing::info;
use twilight_model::{
    guild::{Member, MemberFlags},
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
    user::{PremiumType, User, UserFlags},
    util::{ImageHash, Timestamp},
};

use crate::{DatabaseUser, DbMember, LuroDatabase, LuroMember, LuroUserData, LuroUserType};

mod fetch_character;
mod fetch_characters;
mod fetch_marriages;
mod new;
mod update_character;
mod update_character_prefix;
mod update_character_text;

/// A warpper around [User], with [Member] details if [Id<GuildMarker>] was present on type creation.
/// Details are primarily fetched from the database, but this type can be instanced from a [User] / [Member] if that fails.
/// Also holds some additional which are relevent to Luro only. These are empty if the type was not instanced from the database.
///
/// Check [LuroUserType] to know how this type was instanced.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LuroUser {
    pub data: Option<LuroUserData>,
    pub member: Option<LuroMember>,
    pub instance: LuroUserType,
    pub accent_colour: Option<i32>,
    pub avatar_decoration: Option<String>,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub bot: bool,
    pub discriminator: i16,
    pub email: Option<String>,
    pub flags: Option<i64>,
    pub global_name: Option<String>,
    pub locale: Option<String>,
    pub mfa_enabled: Option<bool>,
    pub name: String,
    pub premium_type: Option<i16>,
    pub public_flags: Option<i64>,
    pub system: Option<bool>,
    pub user_id: i64,
    pub verified: Option<bool>,
}

impl From<DatabaseUser> for LuroUser {
    fn from(user: DatabaseUser) -> Self {
        Self {
            instance: LuroUserType::DbUser,
            data: Some(LuroUserData {
                permissions: user.user_permissions,
            }),
            member: None,
            accent_colour: user.accent_colour,
            avatar_decoration: user.avatar_decoration.map(|x| ImageHash::parse(x.as_bytes()).unwrap()),
            avatar: match user.user_avatar {
                Some(x) => match ImageHash::parse(x.as_bytes()) {
                    Ok(avatar) => Some(avatar),
                    Err(_) => None,
                },
                None => None,
            },
            banner: match user.banner {
                Some(x) => match ImageHash::parse(x.as_bytes()) {
                    Ok(avatar) => Some(avatar),
                    Err(_) => None,
                },
                None => None,
            },
            bot: user.bot,
            discriminator: user.discriminator,
            email: user.email,
            flags: user.user_flags,
            global_name: user.global_name,
            locale: user.locale,
            mfa_enabled: user.mfa_enabled,
            name: user.name,
            premium_type: user.premium_type,
            public_flags: user.public_flags,
            system: user.system,
            verified: user.verified,
            user_id: user.user_id,
        }
    }
}

impl From<DbMember> for LuroUser {
    fn from(member: DbMember) -> Self {
        Self {
            instance: LuroUserType::DbMember,
            data: Some(LuroUserData {
                permissions: member.user_permissions,
            }),
            member: Some(LuroMember {
                user_id: member.user_id,
                guild_id: member.guild_id,
                avatar: member.guild_avatar.as_ref().map(|x| ImageHash::parse(x.as_bytes()).unwrap()),
                boosting_since: member.boosting_since,
                communication_disabled_until: member.communication_disabled_until,
                deafened: member.deafened,
                flags: member.member_flags,
                muted: member.muted,
                nickname: member.nickname,
                pending: member.pending,
                joined_at: member.joined_at.unwrap(),
                roles: member.roles.unwrap().into_iter().map(|x| Id::new(x as u64)).collect::<Vec<_>>(),
            }),
            accent_colour: member.accent_colour,
            avatar_decoration: member.avatar_decoration.map(|x| ImageHash::parse(x.as_bytes()).unwrap()),
            avatar: member.user_avatar.as_ref().map(|x| ImageHash::parse(x.as_bytes()).unwrap()),
            banner: member.banner.map(|x| ImageHash::parse(x.as_bytes()).unwrap()),
            bot: member.bot,
            discriminator: member.discriminator,
            email: member.email,
            flags: member.user_flags,
            global_name: member.global_name,
            locale: member.locale,
            mfa_enabled: member.mfa_enabled,
            name: member.name,
            premium_type: member.premium_type,
            public_flags: member.public_flags,
            system: member.system,
            user_id: member.user_id,
            verified: member.verified,
        }
    }
}

impl From<User> for LuroUser {
    fn from(user: User) -> Self {
        Self {
            instance: LuroUserType::User,
            member: None,
            data: None,
            accent_colour: user.accent_color.map(|x| x as i32),
            avatar_decoration: user.avatar_decoration,
            avatar: user.avatar,
            banner: user.banner,
            bot: user.bot,
            discriminator: user.discriminator as i16,
            email: user.email,
            flags: user.flags.map(|x| x.bits() as i64),
            global_name: user.global_name,
            locale: user.locale,
            mfa_enabled: user.mfa_enabled,
            name: user.name,
            premium_type: user.premium_type.map(|x| u8::from(x) as i16),
            public_flags: user.public_flags.map(|x| x.bits() as i64),
            system: user.system,
            verified: user.verified,
            user_id: user.id.get() as i64,
        }
    }
}

impl From<(Member, Id<GuildMarker>)> for LuroUser {
    fn from((member, guild_id): (Member, Id<GuildMarker>)) -> Self {
        Self {
            instance: LuroUserType::Member,
            data: None,
            member: Some(LuroMember {
                avatar: member.avatar,
                boosting_since: match member.premium_since {
                    Some(timestamp) => Some(OffsetDateTime::from_unix_timestamp(timestamp.as_secs()).unwrap()),
                    None => None,
                },
                communication_disabled_until: match member.communication_disabled_until {
                    Some(timestamp) => Some(OffsetDateTime::from_unix_timestamp(timestamp.as_secs()).unwrap()),
                    None => None,
                },
                joined_at: OffsetDateTime::from_unix_timestamp(member.joined_at.as_secs()).unwrap(),
                deafened: member.deaf,
                flags: member.flags.bits() as i64,
                guild_id: guild_id.get() as i64,
                muted: member.mute,
                nickname: member.nick,
                pending: member.pending,
                roles: member.roles,
                user_id: member.user.id.get() as i64,
            }),
            accent_colour: member.user.accent_color.map(|x| x as i32),
            avatar_decoration: member.user.avatar_decoration,
            avatar: member.user.avatar,
            banner: member.user.banner,
            bot: member.user.bot,
            discriminator: member.user.discriminator as i16,
            email: member.user.email,
            flags: member.user.flags.map(|x| x.bits() as i64),
            global_name: member.user.global_name,
            locale: member.user.locale,
            mfa_enabled: member.user.mfa_enabled,
            name: member.user.name.clone(),
            premium_type: member.user.premium_type.map(|x| u8::from(x) as i16),
            public_flags: member.user.public_flags.map(|x| x.bits() as i64),
            system: member.user.system,
            verified: member.user.verified,
            user_id: member.user.id.get() as i64,
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

    /// Returns a [Id<UserMarker>].
    pub fn user_id(&self) -> Id<UserMarker> {
        Id::new(self.user_id as u64)
    }

    /// Return a string that is a link to the user's avatar
    pub fn avatar(&self) -> String {
        let user_id = self.user_id;
        if let Some(member) = &self.member {
            let guild_id = member.guild_id;

            if let Some(avatar) = member.avatar {
                info!("Guild Avatar: {:#?}", avatar.to_string());

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
            None => format!("https://cdn.discordapp.com/avatars/{}.png?size=2048", self.user_id > 22 % 6),
        }
    }

    /// Return a string that is a link to the user's banner, or [None] if they don't have one
    pub fn banner(&self) -> Option<String> {
        self.banner.map(|banner| match banner.is_animated() {
            true => format!("https://cdn.discordapp.com/banners/{}/{}.gif?size=4096", self.user_id, banner),
            false => format!("https://cdn.discordapp.com/banners/{}/{}.png?size=4096", self.user_id, banner),
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

    /// Write back any changes to the database
    pub async fn push_changes(&self, db: Arc<LuroDatabase>) -> anyhow::Result<LuroUser> {
        Ok(db.update_user(self.clone()).await.map(|x| x.into())?)
    }
}

impl From<LuroUser> for User {
    fn from(luro_user: LuroUser) -> Self {
        Self {
            accent_color: luro_user.accent_colour.map(|x| x as u32),
            avatar_decoration: luro_user.avatar_decoration,
            avatar: luro_user.avatar,
            banner: luro_user.banner,
            bot: luro_user.bot,
            discriminator: luro_user.discriminator as u16,
            email: luro_user.email,
            flags: luro_user.flags.map(|x| UserFlags::from_bits_retain(x as u64)),
            global_name: luro_user.global_name,
            locale: luro_user.locale,
            mfa_enabled: luro_user.mfa_enabled,
            name: luro_user.name,
            premium_type: luro_user.premium_type.map(|x| PremiumType::from(x as u8)),
            public_flags: luro_user.public_flags.map(|x| UserFlags::from_bits_retain(x as u64)),
            system: luro_user.system,
            verified: luro_user.verified,
            id: Id::new(luro_user.user_id as u64),
        }
    }
}

impl TryFrom<LuroUser> for Member {
    type Error = anyhow::Error;

    fn try_from(luro_user: LuroUser) -> Result<Self, Self::Error> {
        match luro_user.member {
            Some(ref member) => Ok(Self {
                avatar: member.avatar,
                communication_disabled_until: match member.communication_disabled_until {
                    Some(timestamp) => Some(Timestamp::from_secs(timestamp.unix_timestamp())?),
                    None => None,
                },
                deaf: member.deafened,
                flags: MemberFlags::from_bits_retain(member.flags as u64),
                joined_at: Timestamp::from_secs(member.joined_at.unix_timestamp())?,
                mute: member.muted,
                nick: member.nickname.clone(),
                pending: member.pending,
                premium_since: match member.boosting_since {
                    Some(timestamp) => Some(Timestamp::from_secs(timestamp.unix_timestamp())?),
                    None => None,
                },
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
