use std::convert::TryInto;

use tracing::warn;
use twilight_model::{
    guild::Member,
    id::{
        marker::{GuildMarker, UserMarker},
        Id
    },
    user::{CurrentUser, User},
    util::ImageHash
};
use twilight_util::builder::embed::{image_source::ImageSourceUrlError, ImageSource};

use crate::LuroContext;

use super::SlashUser;

impl TryInto<ImageSource> for SlashUser {
    type Error = ImageSourceUrlError;

    fn try_into(mut self) -> Result<ImageSource, Self::Error> {
        self.avatar();
        ImageSource::url(self.avatar)
    }
}

impl From<CurrentUser> for SlashUser {
    fn from(user: CurrentUser) -> Self {
        let mut slash_user = Self {
            user_id: user.id,
            user_avatar: user.avatar,
            user_banner: user.banner,
            user_name: user.name.clone(),
            user_discriminator: user.discriminator,
            member_avatar: None,
            guild_id: None,
            avatar: "".to_owned(),
            banner: None,
            name: "".to_owned()
        };

        slash_user.avatar().banner().name();
        slash_user
    }
}

impl From<User> for SlashUser {
    fn from(user: User) -> Self {
        let mut slash_user = Self {
            user_id: user.id,
            user_avatar: user.avatar,
            user_banner: user.banner,
            user_name: user.name.clone(),
            user_discriminator: user.discriminator,
            member_avatar: None,
            guild_id: None,
            avatar: "".to_owned(),
            banner: None,
            name: "".to_owned()
        };

        slash_user.avatar().banner().name();
        slash_user
    }
}

impl From<&User> for SlashUser {
    fn from(user: &User) -> Self {
        let mut slash_user = Self {
            user_id: user.id,
            user_avatar: user.avatar,
            user_banner: user.banner,
            user_name: user.name.clone(),
            user_discriminator: user.discriminator,
            member_avatar: None,
            guild_id: None,
            avatar: "".to_owned(),
            banner: None,
            name: "".to_owned()
        };

        slash_user.avatar().banner().name();
        slash_user
    }
}

impl SlashUser {
    /// Make sure all our data is formatted
    pub fn format(&mut self) -> &mut Self {
        self.avatar().banner().name();
        self
    }

    /// Fetch a member using the client. Useful for when you need some additional information
    pub async fn client_fetch_member(
        ctx: &LuroContext,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>
    ) -> anyhow::Result<(Member, Self)> {
        let member = ctx.twilight_client.guild_member(guild_id, user_id).await?.model().await?;

        let mut slash_user = Self {
            user_id,
            user_avatar: member.user.avatar,
            user_banner: member.user.banner,
            user_name: member.user.name.clone(),
            user_discriminator: member.user.discriminator,
            member_avatar: member.avatar,
            guild_id: Some(guild_id),
            avatar: "".to_owned(),
            name: "".to_owned(),
            banner: None
        };

        slash_user.format();
        Ok((member, slash_user))
    }

    /// Fetch a user using the client. Useful for when you need some additional information
    pub async fn client_fetch_user(ctx: &LuroContext, user_id: Id<UserMarker>) -> anyhow::Result<(User, Self)> {
        let user = ctx.twilight_client.user(user_id).await?.model().await?;

        let mut slash_user = Self {
            user_id,
            user_avatar: user.avatar,
            user_banner: user.banner,
            user_name: user.name.clone(),
            user_discriminator: user.discriminator,
            member_avatar: None,
            guild_id: None,
            avatar: "".to_owned(),
            name: "".to_owned(),
            banner: None
        };

        slash_user.format();
        Ok((user, slash_user))
    }

    /// Return some information from an existing or already fetched member
    pub fn from_member(user: &User, member_avatar: Option<ImageHash>, guild_id: Option<Id<GuildMarker>>) -> Self {
        let mut slash_user = Self {
            user_id: user.id,
            user_avatar: user.avatar,
            user_banner: user.banner,
            user_name: user.name.clone(),
            user_discriminator: user.discriminator,
            member_avatar,
            guild_id,
            avatar: "".to_owned(),
            banner: None,
            name: "".to_owned()
        };

        slash_user.format();
        slash_user
    }

    /// Return a string that is a link to the user's avatar
    pub fn avatar(&mut self) -> &mut Self {
        let avatar = if let Some(guild_id) = self.guild_id && let Some(member_avatar) = self.member_avatar {
            match member_avatar.is_animated() {
                true => format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{}/avatars/{member_avatar}.gif?size=2048", self.user_id),
                false => format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{}/avatars/{member_avatar}.png?size=2048", self.user_id),
            }
        } else {
            match self.user_avatar {
                Some(avatar) => {
                    match avatar.is_animated() {
                        true => format!("https://cdn.discordapp.com/avatars/{}/{avatar}.gif?size=2048", self.user_id),
                        false => format!("https://cdn.discordapp.com/avatars/{}/{avatar}.png?size=2048", self.user_id)
                    }
                },
                None => format!("https://cdn.discordapp.com/embed/avatars/{}.png?size=2048", self.user_discriminator % 5),
            }
        };

        self.avatar = avatar;
        self
    }

    /// Return a string that is a link to the user's banner, or [None] if they don't have one
    pub fn banner(&mut self) -> &mut Self {
        let banner = if let Some(banner) = self.user_banner {
            match banner.is_animated() {
                true => Some(format!(
                    "https://cdn.discordapp.com/banners/{}/{banner}.gif?size=4096",
                    self.user_id
                )),
                false => Some(format!(
                    "https://cdn.discordapp.com/banners/{}/{banner}.png?size=4096",
                    self.user_id
                ))
            }
        } else {
            None
        };

        self.banner = banner;
        self
    }

    pub fn name(&mut self) -> &mut Self {
        self.name = if self.user_discriminator == 0 {
            self.user_name.clone()
        } else {
            format!("{}#{}", self.user_name, self.user_discriminator)
        };
        self
    }

    /// Attempts to fetch the member of the supplied guild_id, otherwise returns the user. This JUST returns the slash_user context.
    pub async fn client_fetch(
        ctx: &LuroContext,
        guild_id: Option<Id<GuildMarker>>,
        user_id: Id<UserMarker>
    ) -> anyhow::Result<Self> {
        match guild_id {
            Some(guild_id) => match Self::client_fetch_member(&ctx, guild_id, user_id).await {
                Ok(member) => Ok(member.1),
                Err(_) => Ok(SlashUser::client_fetch_user(&ctx, user_id).await?.1)
            },
            None => Ok(SlashUser::client_fetch_user(&ctx, user_id).await?.1)
        }
    }
}
