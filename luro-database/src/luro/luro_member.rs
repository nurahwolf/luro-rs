use std::sync::Arc;

use serde::{Deserialize, Serialize};
use twilight_model::{
    guild::{MemberFlags, Permissions},
    id::{
        marker::{GuildMarker, RoleMarker, UserMarker},
        Id,
    },
    util::{ImageHash, Timestamp},
};
use twilight_util::permission_calculator::PermissionCalculator;


use crate::{LuroMemberData, LuroGuild, LuroDatabase};

/// A warpper around [User], with [Member] details if [Id<GuildMarker>] was present on type creation.
/// Details are primarily fetched from the database, but this type can be instanced from a [User] / [Member] if that fails.
/// Also holds some additional which are relevent to Luro only. These are empty if the type was not instanced from the database.
///
/// Check [LuroUserType] to know how this type was instanced.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LuroMember {
    pub avatar: Option<ImageHash>,
    pub boosting_since: Option<Timestamp>,
    pub communication_disabled_until: Option<Timestamp>,
    /// Only present when fetched from the database
    pub data: Option<LuroMemberData>,
    pub deafened: bool,
    pub flags: MemberFlags,
    pub guild_id: Id<GuildMarker>,
    pub joined_at: Timestamp,
    pub muted: bool,
    pub nickname: Option<String>,
    pub pending: bool,
    pub roles: Vec<Id<RoleMarker>>,
    pub user_id: Id<UserMarker>,
}

impl LuroMember {
    /// Fetches the member's permission calculator
    ///
    /// TODO: Remove the member_roles parameter
    pub async fn permission_calculator<'a>(
        &'a self,
        db: Arc<LuroDatabase>,
        member_roles: &'a [(Id<RoleMarker>, Permissions)],
    ) -> anyhow::Result<PermissionCalculator> {
        if let Some(ref data) = self.data {
            return data.permission_calculator(db, member_roles).await
        }
        
        let guild = LuroGuild::new(db.clone(), self.guild_id).await?;
        let everyone_role = guild.get_everyone_role(db).await?;

        Ok(PermissionCalculator::new(self.guild_id, self.user_id, everyone_role.permissions, member_roles).owner_id(guild.owner_id))
    }

    /// Gets all roles and their permissions, excluding the everyone role
    pub fn role_permissions(&self) -> Vec<(Id<RoleMarker>, Permissions)> {
        if let Some(ref data) = self.data {
            return data.role_permissions()
        }

        todo!()
    }
}