#![feature(let_chains)]

use itertools::Itertools;
use poise::serenity_prelude::{Colour, Guild, RoleId, Role};

/// Get the guild accent colour. If no guild is specified, or we fail to get the highest role, fall back to our defined accent colour
pub fn guild_accent_colour(accent: [u8; 3], guild: Option<Guild>) -> Colour {
    if let Some(guild) = guild {
        if let Some(highest_role) = sort_roles(&guild).first() && highest_role.1.colour.0 != 0 {
            return highest_role.1.colour;
        };
    };

    accent_colour(accent)
}

/// Instead of getting a guild accent colour like in [guild_accent_colour], this function just returns the one from the config, or passed through as a RGB array
pub fn accent_colour(accent: [u8; 3]) -> Colour {
    Colour::from_rgb(accent[0], accent[1], accent[2])
}

pub fn sort_roles(guild: &Guild) -> Vec<(&RoleId, &Role)> {
    guild.roles.iter().sorted_by_key(|&(_, r)| -r.position).collect::<Vec<_>>()
}

/// Converts integers to human-readable integers separated by
/// commas, e.g. "1000000" displays as "1,000,000" when fed through
/// this function.
pub fn format_int(int: u64) -> String {
    let mut string = String::new();
    for (idx, val) in int.to_string().chars().rev().enumerate() {
        if idx != 0 && idx % 3 == 0 {
            string.insert(0, ',');
        }
        string.insert(0, val);
    }
    string
}
