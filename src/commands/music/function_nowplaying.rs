use chrono::DateTime;
use poise::serenity_prelude::{CreateEmbed, Guild, User};
use songbird::input::Metadata;

use crate::utils::guild_accent_colour;

pub fn now_playing(accent_colour: [u8; 3], guild: Guild, user: Option<User>, metadata: &Metadata) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.title("Now Playing");
    embed.color(guild_accent_colour(accent_colour, Some(guild)));
    if let Some(user) = user {
        embed.field("Requested By", user, false);
    }
    if let Some(title) = &metadata.title {
        embed.title(title);
    }
    if let Some(artist) = &metadata.artist {
        embed.field("Arist", artist, false);
    }
    if let Some(date) = &metadata.date {
        match DateTime::parse_from_str(date, "%Y%m%d") {
            Ok(date_parsed) => embed.field("Date", format!("<t:{}:D>",date_parsed.timestamp()), false),
            Err(_) => embed.field("Date", date, false),
        };
    }
    if let Some(duration) = &metadata.duration {
        embed.field("Duration", duration.as_secs(), false);
    }
    if let Some(source) = &metadata.source_url {
        embed.field("Source", source, false);
    }
    if let Some(start_time) = &metadata.start_time {
        embed.field("Start Time", format!("Time in seconds: {}", start_time.as_secs()), false);
    }
    if let Some(track) = &metadata.track {
        embed.field("Track", track, false);
    }
    if let Some(thumbnail) = &metadata.thumbnail {
        embed.thumbnail(thumbnail);
    }
    embed
}
