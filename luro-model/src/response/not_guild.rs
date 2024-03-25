use twilight_model::id::{marker::GuildMarker, Id};

use crate::builders::EmbedBuilder;

use super::Response;

/// A response that is raised if this command can only be executed in a guild.
///
/// Parameters:
///
/// - guild_id: What guild should have been present.
/// - colour: The colour for the embed, otherwise returns the hardcoded default.
pub fn not_guild(colour: Option<u32>, guild_id: Option<Id<GuildMarker>>) -> EmbedBuilder {
    let mut embed = EmbedBuilder::default();
    embed.title("Guild Not Found!").colour(colour.unwrap_or(crate::COLOUR_ERROR));

    match guild_id {
        Some(guild_id) => embed.description(format!("I expected to locate guild `{guild_id}`, but it could not be found.")),
        None => embed.description("This interaction requires you to be in a guild, or for me to be able to get guild data."),
    };
    embed
}

impl<'a> Response<'a> {
    pub fn not_guild(&self) -> EmbedBuilder {
        not_guild(Some(self.accent_colour()), self.guild_id())
    }
}
