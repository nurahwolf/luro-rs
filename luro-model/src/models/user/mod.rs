use twilight_model::{
    guild::{Permissions, Role},
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
};

mod member_context;
mod user_context;
mod user_permissions;

pub use member_context::MemberContext;
pub use user_context::UserContext;
pub use user_permissions::UserPermissions;

use crate::database::{Database, Error};

/// Luro's user object. This is an enum for two primary states:
///
/// - Member: A member context spawned where there is guild information available, such as in a guild.
/// - User: A user context spawned where there is NO guild information available.
///
/// There are several helper functions available for getting things like the user's preferred name, or user ID.
pub enum User {
    /// A member context spawned where there is guild information available, such as in a guild.
    Member(MemberContext),
    /// A user context spawned where there is NO guild information available.
    User(UserContext),
}

impl User {
    ///Returns a reference to the contained [MemberContext], if present.
    pub fn member_context_optional(&self) -> Option<&MemberContext> {
        match self {
            User::Member(member) => Some(member),
            User::User(_) => None,
        }
    }

    /// Using the database, update this object with information from the passed guild.
    ///
    /// This changes the type to a [MemberContext], and returns a reference to it for convenience.
    pub async fn member_context(&mut self, db: &Database, guild_id: Id<GuildMarker>) -> Result<&MemberContext, Error> {
        if let Self::Member(ref member) = self {
            return Ok(member);
        }

        *self = User::Member(db.fetch_member(guild_id, self.user_id()).await?);
        match self {
            User::Member(ref member) => Ok(member),
            _ => unreachable!(),
        }
    }

    /// Return the contained user context for this instance.
    pub fn user_context(&self) -> &UserContext {
        match self {
            User::Member(member) => &member.user,
            User::User(user) => user,
        }
    }

    pub fn user_id(&self) -> Id<UserMarker> {
        match self {
            Self::User(user) => user.twilight_user.id,
            Self::Member(member) => member.twilight_member.user.id,
        }
    }

    /// Get's the user's username name
    ///
    /// Returns the first match
    /// Username -> Legacy Username
    pub fn username(&self) -> String {
        match self {
            Self::User(user) => user.username(),
            Self::Member(member) => member.username(),
        }
    }

    /// Return a string that is a link to the user's avatar
    pub fn avatar_url(&self) -> String {
        match self {
            Self::User(user) => user.avatar_url(),
            Self::Member(member) => member.avatar_url(),
        }
    }

    pub fn banner(&self) -> Option<twilight_model::util::ImageHash> {
        match self {
            Self::User(user) => user.twilight_user.banner,
            Self::Member(member) => member.twilight_member.user.banner,
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

    pub fn global_name(&self) -> Option<&str> {
        match self {
            User::Member(member) => member.twilight_member.user.global_name.as_deref(),
            User::User(user) => user.twilight_user.global_name.as_deref(),
        }
    }

    /// Get's the user's preferred / pretty name
    ///
    /// Returns the first match
    /// Member Nickname -> Global Name -> Username -> Legacy Username
    pub fn name(&self) -> String {
        if let Self::Member(member) = self {
            if let Some(ref nickname) = member.twilight_member.nick {
                return nickname.clone();
            }
        }
        self.global_name().map(|x| x.to_owned()).unwrap_or_else(|| self.username())
    }

    /// Internally create a permission calculator, then return the member's highest role and overall permissions.
    pub fn permission_matrix_highest_role(&self, owner_id: Id<UserMarker>) -> Option<(Option<&Role>, Permissions)> {
        match self {
            User::Member(member) => Some((member.roles.first(), member.permission_matrix(owner_id))),
            User::User(_) => None,
        }
    }
}

impl From<UserContext> for User {
    fn from(user: UserContext) -> Self {
        Self::User(user)
    }
}

impl From<MemberContext> for User {
    fn from(member: MemberContext) -> Self {
        Self::Member(member)
    }
}

impl From<User> for twilight_model::user::User {
    fn from(user: User) -> Self {
        match user {
            User::Member(member) => member.twilight_member.user,
            User::User(user) => user.twilight_user,
        }
    }
}
