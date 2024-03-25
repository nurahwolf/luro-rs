use twilight_model::{
    guild::{Member, Permissions, Role},
    id::{
        marker::{GuildMarker, RoleMarker, UserMarker},
        Id,
    },
};
use twilight_util::permission_calculator::PermissionCalculator;

use crate::{
    database::{Database, Error},
    user::UserContext,
};

/// A context spawned around a member.
pub struct MemberContext {
    pub user: UserContext,
    pub twilight_member: twilight_model::guild::Member,
    pub guild_id: Id<GuildMarker>,
    pub roles: Vec<Role>,            // All roles excluding the everyone role
    pub everyone_role: Option<Role>, // The everyone role
}

impl MemberContext {
    pub fn user_id(&self) -> Id<UserMarker> {
        self.user.twilight_user.id
    }

    /// Get's the user's username name
    ///
    /// Returns the first match
    /// Username -> Legacy Username
    pub fn username(&self) -> String {
        match self.twilight_member.user.discriminator == 0 {
            true => self.twilight_member.user.name.clone(),
            false => format!("{}#{}", self.twilight_member.user.name, self.twilight_member.user.discriminator),
        }
    }

    /// Return a string that is a link to the user's avatar
    pub fn avatar_url(&self) -> String {
        let id = self.twilight_member.user.id;

        if let Some(member_avatar) = self.twilight_member.avatar {
            let guild_id = self.guild_id;
            return match member_avatar.is_animated() {
                true => format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{id}/avatars/{member_avatar}.gif?size=2048"),
                false => format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{id}/avatars/{member_avatar}.png?size=2048"),
            };
        }

        self.user.avatar_url()
    }

    /// Internally create a permission calculator, then return the member's highest role and overall permissions.
    pub fn permission_matrix_highest_role(&self, owner_id: Id<UserMarker>) -> (Option<&Role>, Permissions) {
        (self.roles.first(), self.permission_matrix(owner_id))
    }

    /// Internally create a permission calculator and return the member's overall permissions.
    pub fn permission_matrix(&self, owner_id: Id<UserMarker>) -> Permissions {
        self.permission_calculator(&self.role_permissions()).owner_id(owner_id).root()
    }

    /// Create a permission calculator.
    pub fn permission_calculator<'a>(&'a self, permissions: &'a [(Id<RoleMarker>, Permissions)]) -> PermissionCalculator {
        let guild_id = self.guild_id;
        let user_id = self.user_id();
        let everyone_role = match self.everyone_role {
            Some(ref everyone_role) => everyone_role.permissions,
            None => Permissions::empty(),
        };

        PermissionCalculator::new(guild_id, user_id, everyone_role, permissions)
    }

    /// Return just the role IDs and permissions of the member
    pub fn role_permissions(&self) -> Vec<(Id<RoleMarker>, Permissions)> {
        self.roles.iter().map(|role| (role.id, role.permissions)).collect()
    }

    /// By default we just have role IDs, use this to fetch actual role data.
    pub async fn sync_roles<'a>(&'a mut self, db: &'a Database) -> Result<&'a mut MemberContext, Error> {
        db.fetch_member_roles(self).await
    }
}

impl From<(Id<GuildMarker>, Member)> for MemberContext {
    fn from((guild_id, twilight_member): (Id<GuildMarker>, Member)) -> Self {
        Self {
            user: twilight_member.user.clone().into(),
            twilight_member,
            guild_id,
            roles: vec![],
            everyone_role: None,
        }
    }
}
