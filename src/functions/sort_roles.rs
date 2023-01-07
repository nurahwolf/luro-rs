use itertools::Itertools;
use poise::serenity_prelude::{Guild, RoleId, Role};

pub fn sort_roles(guild: &Guild) -> std::vec::IntoIter<(&RoleId, &Role)> {
    guild.roles.iter().sorted_by_key(|&(_, r)| -r.position)
}