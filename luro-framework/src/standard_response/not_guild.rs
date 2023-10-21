use luro_model::{builders::EmbedBuilder, COLOUR_DANGER};
use tracing::warn;

/// Guild command invoked in not a guild context
pub fn not_guild() -> EmbedBuilder {
    warn!("Guild Command was ran but I could not work out what guild they were in");
    let mut embed = EmbedBuilder::default();
    embed
        .title("Unable to find this guild!")
        .colour(COLOUR_DANGER)
        .description("If you ran this command in a guild, I am unable to find it. If this is a DM... Please tell Nurah to limit this command to guilds only.");
    embed
}
