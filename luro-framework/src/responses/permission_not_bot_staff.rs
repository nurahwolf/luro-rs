use luro_builder::embed::EmbedBuilder;
use luro_model::COLOUR_DANGER;

const INSULTS: [&str; 4] = [
    "Great job motherfucker, you are not the bot owner and do not have permission to use that command.\n\n**THE COMMAND IS LITERALLY NAMED OWNER ONLY! WHAT THE HECK DID YOU THINK WOULD HAPPEN!?**",
    "Dork, this is literally an owner only command. Did you READ what the command was named?",
    "Nice try.",
    "Get fucked."
];

/// An embed for when someone runs a privileged command that they do not have permission for.
pub fn permission_not_bot_staff() -> EmbedBuilder {
    let mut embed = EmbedBuilder::default();
    embed
        .title("You are not the bot owner!")
        .colour(COLOUR_DANGER)
        .description(get_insult())
        .footer(|f| f.text("FYI, I'm reporting you to Nurah."));
    embed
}

/// Gets a insult. This version does not use RNG to get a response.
#[cfg(not(feature = "random-responses"))]
fn get_insult<'a>() -> &'a str {
    INSULTS[0]
}

/// Gets a insult. This version uses [rand] to get an insult.
#[cfg(feature = "random-responses")]
fn get_insult<'a>() -> &'a str {
    use rand::seq::SliceRandom;

    let mut rng = rand::thread_rng();
    INSULTS.choose(&mut rng).unwrap_or(&INSULTS[0])
}
