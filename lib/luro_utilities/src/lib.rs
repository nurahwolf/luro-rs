use itertools::Itertools;
use poise::serenity_prelude::{Colour, Guild, Role, RoleId};

pub fn guild_accent_colour(accent: [u8; 3], guild: Option<Guild>) -> Colour {
    if let Some(guild) = &guild {
        let highest_role = sort_roles(guild).next().expect("No roles in the server, somehow");

        if highest_role.1.colour.0 != 0 {
            return highest_role.1.colour;
        };
    };

    Colour::from_rgb(accent[0], accent[1], accent[2])
}

pub fn accent_colour(accent: [u8; 3]) -> Colour {
    Colour::from_rgb(accent[0], accent[1], accent[2])
}

pub fn sort_roles(guild: &Guild) -> std::vec::IntoIter<(&RoleId, &Role)> {
    guild.roles.iter().sorted_by_key(|&(_, r)| -r.position)
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
