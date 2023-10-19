use std::sync::Arc;

use sqlx::types::Json;
use tracing::info;
use twilight_model::{
    id::{marker::UserMarker, Id},
    user::{PremiumType, UserFlags, User},
    util::ImageHash,
};

use crate::{LuroDatabase, LuroMember, LuroUserType, LuroUserData};

mod fetch_characters;
mod fetch_character;
mod update_character_text;
mod update_character;
mod fetch_marriages;
mod update_character_prefix;

/// A warpper around [User], with [Member] details if [Id<GuildMarker>] was present on type creation.
/// Details are primarily fetched from the database, but this type can be instanced from a [User] / [Member] if that fails.
/// Also holds some additional which are relevent to Luro only. These are empty if the type was not instanced from the database.
///
/// Check [LuroUserType] to know how this type was instanced.
#[derive(Clone, Debug)]
pub struct LuroUser {
    pub data: Option<LuroUserData>,
    pub member: Option<LuroMember>,
    pub instance: LuroUserType,
    pub db: Arc<LuroDatabase>,
    pub accent_colour: Option<i32>,
    pub avatar_decoration: Option<Json<ImageHash>>,
    pub avatar: Option<Json<ImageHash>>,
    pub banner: Option<Json<ImageHash>>,
    pub bot: bool,
    pub discriminator: i16,
    pub email: Option<String>,
    pub flags: Option<Json<UserFlags>>,
    pub global_name: Option<String>,
    pub locale: Option<String>,
    pub mfa_enabled: Option<bool>,
    pub name: String,
    pub premium_type: Option<Json<PremiumType>>,
    pub public_flags: Option<Json<UserFlags>>,
    pub system: Option<bool>,
    pub user_id: i64,
    pub verified: Option<bool>,
}

impl LuroUser {
    /// Update the contained member.
    pub fn update_member(&mut self, member: impl Into<LuroMember>) -> &mut Self {
        let member = member.into();
        self.member = Some(member);
        self
    }

    /// Sync any changes with the database
    pub async fn sync(&mut self) -> anyhow::Result<&mut Self> {
        self.db.update_user(self.clone()).await?;
        Ok(self)
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

            if let Some(avatar) = member.avatar.map(|x| x.0) {
                info!("Guild Avatar: {:#?}", avatar.to_string());

                return match avatar.is_animated() {
                    true => format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{avatar}.gif?size=2048"),
                    false => format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{avatar}.png?size=2048"),
                };
            }
        }

        match self.avatar.map(|x| x.0) {
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
            true => format!("https://cdn.discordapp.com/banners/{}/{}.gif?size=4096", self.user_id, banner.0),
            false => format!("https://cdn.discordapp.com/banners/{}/{}.png?size=4096", self.user_id, banner.0),
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
    pub async fn push_changes(&self) -> anyhow::Result<()> {
        self.db.update_user(self.clone()).await?;

        if let Some(member) = self.member.clone() {
            self.db.update_member(member).await?;
        }

        if let Some(data) = self.data.clone() {
            self.db.update_user_data(self.user_id, data).await?;
        }

        Ok(())
    }
}

impl From<LuroUser> for User {
    fn from(luro_user: LuroUser) -> Self {
        Self {
            accent_color: luro_user.accent_colour.map(|x| x as u32),
            avatar_decoration: luro_user.avatar_decoration.map(|x|x.0),
            avatar: luro_user.avatar.map(|x|x.0),
            banner: luro_user.banner.map(|x|x.0),
            bot: luro_user.bot,
            discriminator: luro_user.discriminator as u16,
            email: luro_user.email,
            flags: luro_user.flags.map(|x|x.0),
            global_name: luro_user.global_name,
            locale: luro_user.locale,
            mfa_enabled: luro_user.mfa_enabled,
            name: luro_user.name,
            premium_type: luro_user.premium_type.map(|x|x.0),
            public_flags: luro_user.public_flags.map(|x|x.0),
            system: luro_user.system,
            verified: luro_user.verified,
            id: Id::new(luro_user.user_id as u64),
        }
    }
}