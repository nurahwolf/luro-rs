use twilight_model::{
    gateway::payload::incoming::{RoleCreate, RoleDelete, RoleUpdate},
    id::{
        marker::{GuildMarker, RoleMarker},
        Id,
    },
};

pub enum RoleSync<'a> {
    // Role(&'a Role),
    RoleCreate(&'a RoleCreate),
    RoleDelete(&'a RoleDelete),
    RoleId(Id<GuildMarker>, Id<RoleMarker>),
    RoleUpdate(&'a RoleUpdate),
    TwilightRole(Id<GuildMarker>, &'a twilight_model::guild::Role),
}

impl<'a> From<(Id<GuildMarker>, Id<RoleMarker>)> for RoleSync<'a> {
    fn from((guild_id, role_id): (Id<GuildMarker>, Id<RoleMarker>)) -> Self {
        Self::RoleId(guild_id, role_id)
    }
}

// impl<'a> From<&'a Role> for RoleSync<'a> {
//     fn from(role: &'a Role) -> Self {
//         Self::Role(role)
//     }
// }

impl<'a> From<&'a RoleCreate> for RoleSync<'a> {
    fn from(role: &'a RoleCreate) -> Self {
        Self::RoleCreate(role)
    }
}

impl<'a> From<&'a RoleUpdate> for RoleSync<'a> {
    fn from(role: &'a RoleUpdate) -> Self {
        Self::RoleUpdate(role)
    }
}

impl<'a> From<&'a RoleDelete> for RoleSync<'a> {
    fn from(role: &'a RoleDelete) -> Self {
        Self::RoleDelete(role)
    }
}

impl<'a> From<(Id<GuildMarker>, &'a twilight_model::guild::Role)> for RoleSync<'a> {
    fn from((role_id, role): (Id<GuildMarker>, &'a twilight_model::guild::Role)) -> Self {
        Self::TwilightRole(role_id, role)
    }
}
