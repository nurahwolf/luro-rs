use luro_builder::embed::EmbedBuilder;
use luro_model::COLOUR_DANGER;
use tracing::warn;
use twilight_model::id::{marker::UserMarker, Id};

/// A response for if someone tried to screw with a server owner
pub fn permission_server_owner(user_id: &Id<UserMarker>) -> EmbedBuilder {
    warn!("User {user_id} attempted to abuse a server owner");
    let mut embed = EmbedBuilder::default();
    embed
        .colour(COLOUR_DANGER)
        .title("That's the server owner you idiot")
        .description(format!(
            "Congratulations <@{user_id}>, that's the server owner. Do you really think I'm gonna screw with them? Nope!"
        ));
    embed
}
