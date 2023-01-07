use poise::serenity_prelude::{Colour, Guild};

use super::sort_roles::sort_roles;

pub fn guild_accent_colour(accent: [u8; 3], guild: Option<Guild>) -> Colour {
    if let Some(guild) = &guild {
        let highest_role = sort_roles(guild).next().expect("No roles in the server, somehow");

        if highest_role.1.colour.0 != 0 {
            return highest_role.1.colour;
        };
    };

    Colour::from_rgb(accent[0], accent[1], accent[2])
}
