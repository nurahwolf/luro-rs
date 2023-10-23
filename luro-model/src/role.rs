use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
};

use serde::{Deserialize, Serialize};
use twilight_model::{
    guild::{Permissions, Role, RoleFlags, RoleTags},
    id::{
        marker::{GuildMarker, RoleMarker},
        Id,
    },
    util::ImageHash,
};


/// A [BTreeMap] of [RoleMarker], keyed by [usize]
pub type LuroRolePositions = BTreeMap<usize, Id<RoleMarker>>;


