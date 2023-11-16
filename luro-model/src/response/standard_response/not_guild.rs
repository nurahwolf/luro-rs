/// Guild command invoked in not a guild context
pub fn not_guild() -> crate::builders::EmbedBuilder {
    tracing::warn!("Guild Command was ran but I could not work out what guild they were in");
    let mut embed = crate::builders::EmbedBuilder::default();
    embed
        .title("Unable to find this guild!")
        .colour(crate::COLOUR_DANGER)
        .description("If you ran this command in a guild, I am unable to find it. If this is a DM... Please tell Nurah to limit this command to guilds only.");
    embed
}
