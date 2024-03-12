mod avatar_url;
mod member;
mod permission_matrix;
mod username;

use twilight_model::{
    id::{marker::UserMarker, Id},
    user::{PremiumType, UserFlags},
    util::ImageHash,
};

/// A wrapper around [UserContext] or [MemberContext], allowing for shared functionality
#[derive(Clone, Debug, serde::Deserialize, PartialEq, serde::Serialize)]
pub enum User {
    User(UserContext),
    Member(super::MemberContext),
}

#[derive(Clone, Debug, serde::Deserialize, PartialEq, serde::Serialize)]
pub struct UserContext {
    pub accent_colour: Option<u32>,
    pub avatar_decoration: Option<ImageHash>,
    pub avatar: Option<ImageHash>,
    pub banner: Option<ImageHash>,
    pub bot: bool,
    pub discriminator: u16,
    pub email: Option<String>,
    pub flags: Option<UserFlags>,
    pub gender: Option<super::Gender>,
    pub global_name: Option<String>,
    pub locale: Option<String>,
    pub mfa_enabled: Option<bool>,
    pub name: String,
    pub permissions: super::UserPermissions,
    pub premium_type: Option<PremiumType>,
    pub public_flags: Option<UserFlags>,
    pub sexuality: Option<super::Sexuality>,
    pub system: Option<bool>,
    pub user_id: Id<UserMarker>,
    pub verified: Option<bool>,
}

impl User {
    pub fn user_context(&self) -> &UserContext {
        match self {
            User::User(user) => &user,
            User::Member(member) => &member.user,
        }
    }

    pub fn member_context(&self) -> Option<&super::MemberContext> {
        match self {
            User::User(_) => None,
            User::Member(member) => Some(member),
        }
    }

    pub fn accent_colour(&self) -> Option<u32> {
        self.user_context().accent_colour
    }

    pub fn user_id(&self) -> Id<UserMarker> {
        self.user_context().user_id
    }

    pub fn global_name(&self) -> Option<&str> {
        self.user_context().global_name.as_deref()
    }

    /// Get's the user's preferred / pretty name
    ///
    /// Returns the first match
    /// Member Nickname -> Global Name -> Username -> Legacy Username
    pub fn name(&self) -> String {
        // Return member nickname, if present
        if let User::Member(member) = self
            && let Some(member_nickname) = &member.nickname
        {
            return member_nickname.clone();
        }

        // Retur global name, if present
        match &self.user_context().global_name {
            Some(global_name) => global_name.clone(),
            None => self.username(),
        }
    }

    pub fn banner(&self) -> Option<twilight_model::util::ImageHash> {
        match self {
            User::User(user) => user.banner,
            User::Member(member) => match member.banner {
                Some(member_banner) => Some(member_banner),
                None => member.user.banner,
            },
        }
    }

    /// Return a string that is a link to the user's banner, or [None] if they don't have one
    pub fn banner_url(&self) -> Option<String> {
        let user_id = self.user_id();
        self.banner().map(|x| match x.is_animated() {
            true => format!("https://cdn.discordapp.com/banners/{user_id}/{x}.gif?size=4096",),
            false => format!("https://cdn.discordapp.com/banners/{user_id}/{x}.png?size=4096",),
        })
    }
}

impl From<twilight_model::user::User> for User {
    fn from(user: twilight_model::user::User) -> Self {
        Self::User(user.into())
    }
}

impl From<twilight_model::user::User> for UserContext {
    fn from(user: twilight_model::user::User) -> Self {
        UserContext {
            accent_colour: user.accent_color,
            avatar: user.avatar,
            banner: user.banner,
            bot: user.bot,
            gender: None,
            permissions: match crate::BOT_OWNERS.contains(&user.id) {
                true => super::UserPermissions::Owner,
                false => super::UserPermissions::User,
            },
            sexuality: None,
            user_id: user.id,
            discriminator: user.discriminator,
            avatar_decoration: user.avatar_decoration,
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
        }
    }
}

impl From<twilight_model::user::CurrentUser> for User {
    fn from(user: twilight_model::user::CurrentUser) -> Self {
        Self::User(UserContext {
            accent_colour: user.accent_color,
            avatar: user.avatar,
            banner: user.banner,
            bot: user.bot,
            gender: None,
            permissions: match crate::BOT_OWNERS.contains(&user.id) {
                true => super::UserPermissions::Owner,
                false => super::UserPermissions::User,
            },
            sexuality: None,
            user_id: user.id,
            discriminator: user.discriminator,
            avatar_decoration: None,
            email: user.email,
            flags: user.flags,
            global_name: None,
            locale: user.locale,
            mfa_enabled: Some(user.mfa_enabled),
            name: user.name,
            premium_type: user.premium_type,
            public_flags: user.public_flags,
            system: Some(false),
            verified: user.verified,
        })
    }
}

impl From<User> for twilight_model::user::User {
    fn from(user: User) -> Self {
        match user {
            User::User(user) => user.into(),
            User::Member(member) => member.user.into(),
        }
    }
}

impl From<UserContext> for twilight_model::user::User {
    fn from(user: UserContext) -> Self {
        twilight_model::user::User {
            accent_color: user.accent_colour,
            avatar: user.avatar,
            avatar_decoration: user.avatar_decoration,
            banner: user.banner,
            bot: user.bot,
            discriminator: user.discriminator,
            email: user.email,
            flags: user.flags,
            global_name: user.global_name,
            id: user.user_id,
            locale: user.locale,
            mfa_enabled: user.mfa_enabled,
            name: user.name,
            premium_type: user.premium_type,
            public_flags: user.public_flags,
            system: user.system,
            verified: user.verified,
        }
    }
}
