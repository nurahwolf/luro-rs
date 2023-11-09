use twilight_model::{id::{Id, marker::{GuildMarker, RoleMarker}}, gateway::payload::incoming::{RoleCreate, RoleDelete, RoleUpdate}};

use crate::types::Role;

pub enum RoleSync {
    Role(Role),
    RoleCreate(RoleCreate),
    RoleDelete(RoleDelete),
    RoleId(Id<GuildMarker>, Id<RoleMarker>),
    RoleUpdate(RoleUpdate),
    TwilightRole(Id<GuildMarker>, twilight_model::guild::Role),
}

impl From<(Id<GuildMarker>, Id<RoleMarker>)> for RoleSync {
    fn from((guild_id, role_id): (Id<GuildMarker>, Id<RoleMarker>)) -> Self {
        Self::RoleId(guild_id, role_id)
    }
}

impl From<Role> for RoleSync {
    fn from(role: Role) -> Self {
        Self::Role(role)
    }
}

impl From<RoleCreate> for RoleSync {
    fn from(role: RoleCreate) -> Self {
        Self::RoleCreate(role)
    }
}

impl From<RoleUpdate> for RoleSync {
    fn from(role: RoleUpdate) -> Self {
        Self::RoleUpdate(role)
    }
}

impl From<RoleDelete> for RoleSync {
    fn from(role: RoleDelete) -> Self {
        Self::RoleDelete(role)
    }
}

impl From<(Id<GuildMarker>, twilight_model::guild::Role)> for RoleSync {
    fn from(role: (Id<GuildMarker>, twilight_model::guild::Role)) -> Self {
        Self::TwilightRole(role.0, role.1)
    }
}