mod boosing_since;
mod communication_disabled_until;
mod joined_at;

use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use twilight_model::{
    guild::Permissions,
    id::{
        marker::{GuildMarker, RoleMarker, UserMarker},
        Id,
    },
    util::{image_hash::ImageHashParseError, ImageHash},
};
use twilight_util::permission_calculator::PermissionCalculator;

use crate::{LuroDatabase, LuroGuild, LuroRole};

/// A warpper around [User], with [Member] details if [Id<GuildMarker>] was present on type creation.
/// Details are primarily fetched from the database, but this type can be instanced from a [User] / [Member] if that fails.
/// Also holds some additional which are relevent to Luro only. These are empty if the type was not instanced from the database.
///
/// Check [LuroUserType] to know how this type was instanced.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LuroMember {
    #[serde(skip)]
    pub left_at: Option<time::OffsetDateTime>,
    pub avatar: Option<String>,
    #[serde(skip)]
    pub boosting_since: Option<time::OffsetDateTime>,
    #[serde(skip)]
    pub communication_disabled_until: Option<time::OffsetDateTime>,
    #[serde(skip, default = "default")]
    pub joined_at: time::OffsetDateTime,
    pub deafened: bool,
    pub flags: i64,
    pub guild_id: i64,
    pub muted: bool,
    pub nickname: Option<String>,
    pub pending: bool,
    pub roles: HashMap<Id<RoleMarker>, LuroRole>,
    pub user_id: i64,
}

fn default() -> time::OffsetDateTime {
    time::OffsetDateTime::now_utc()
}

impl LuroMember {
    pub fn guild_id(&self) -> Id<GuildMarker> {
        Id::new(self.guild_id as u64)
    }

    pub fn user_id(&self) -> Id<UserMarker> {
        Id::new(self.user_id as u64)
    }

    /// Format the internal avatar as an image hash
    pub fn avatar(&self) -> Result<Option<ImageHash>, ImageHashParseError> {
        Ok(match &self.avatar {
            Some(img) => Some(ImageHash::parse(img.as_bytes())?),
            None => None,
        })
    }

    pub fn role_ids(&self) -> Vec<&Id<RoleMarker>> {
        self.roles.keys().collect()
    }

    /// Returns a vector of roles, sorted by the hiararchy
    pub fn sorted_roles(&self) -> Vec<&LuroRole> {
        let mut roles = self.roles.values().collect::<Vec<_>>();
        roles.sort();
        roles
    }

    /// Fetches the member's permission calculator
    ///
    /// TODO: Remove the member_roles parameter
    pub async fn permission_calculator<'a>(
        &'a self,
        db: Arc<LuroDatabase>,
        member_roles: &'a [(Id<RoleMarker>, Permissions)],
    ) -> anyhow::Result<PermissionCalculator> {
        let guild = LuroGuild::new(db.clone(), self.guild_id()).await?;
        let everyone_role = guild.get_everyone_role(db).await?;

        Ok(
            PermissionCalculator::new(self.guild_id(), self.user_id(), everyone_role.permissions, member_roles)
                .owner_id(guild.owner_id()),
        )
    }

    /// Gets all roles and their permissions, excluding the everyone role
    pub fn role_permissions(&self) -> Vec<(Id<RoleMarker>, Permissions)> {
        let mut new_roles = self.roles.clone();
        new_roles.retain(|_, role| role.id != self.guild_id().cast());
        new_roles.values().map(|x| (x.id, x.permissions)).collect()
    }
}
