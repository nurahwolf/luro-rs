use itertools::Itertools;
use poise::serenity_prelude::{Channel, ChannelId, Guild};

// TODO: Remove the unsafe unwrap
pub fn sort_channels(guild: &Guild) -> std::vec::IntoIter<(&ChannelId, &Channel)> {
    guild.channels.iter().sorted_by_key(|&(_, r)| -r.position().unwrap())
}
