use poise::serenity_prelude::Colour;

pub fn accent_colour(accent: [u8; 3]) -> Colour {
    Colour::from_rgb(accent[0], accent[1], accent[2])
}
