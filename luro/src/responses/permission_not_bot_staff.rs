use luro_model::builders::EmbedBuilder;

const INSULTS: [&str; 50] = [
    "Great job motherfucker, you are not the bot owner and do not have permission to use that command.\n\n**THE COMMAND IS LITERALLY NAMED OWNER ONLY! WHAT THE HECK DID YOU THINK WOULD HAPPEN!?**",
    "Dork, this is literally an owner only command. Did you READ what the command was named?",
    "Nice try.",
    "Get fucked.",
    "Wow, you must be a genius to try using an OWNER ONLY command!",
"Congratulations, you've just won the 'I can't read' award!",
"A for effort, F for execution.",
"Oops! You must have mistaken yourself for someone who has permission.",
"News flash: You're not the owner. Shocking, I know.",
"Whoa there, lone wolf! This command is for the pack leader's eyes only.",
"Trying to use an owner only command? Bold strategy, Cotton.",
"Did you think the command name was just a suggestion?",
"Whoa, slow down there cowboy. You're not the owner.",
"Nice try, furball! But you don't have the 'paw-ermission' for that command!",
"Keep dreaming, buddy.",
"You must be new here.",
"You do realize that 'owner only' means you CAN'T use it, right?",
"You might want to get your eyes checked.",
"I'm sorry, did the sign not say 'No Trespassing'?",
"If I had a nickel for every time someone tried to use an owner only command...",
"Do you also enter rooms marked 'Authorized Personnel Only'?",
"Good thing this isn't a security clearance test.",
"Paws off, pup! This command is for the pack leader only.",
"Here's a gold star for trying.",
"Looks like someone's fur-gotten the rules! Owner only command, silly floof!",
"Epic fail, my friend.",
"You know, there's a special place for people who try to use owner only commands.",
"Well, that was embarrassing.",
"Swing and a miss!",
"Do you need a map to navigate the command list?",
"Maybe try a command that you're actually allowed to use next time.",
"Whoa there, Captain Overconfident! This command is for the owner only!",
"Plot twist: You're NOT the owner. Dun Dun Dunnnnn!",
"Surprise! This isn't a free-for-all command buffet.",
"Breaking news: Local user tries to use owner only command, fails hilariously.",
"In a parallel universe, you might be the owner. But not in this one.",
"Do you also try to use the staff bathroom at restaurants?",
"Hold on, let me check... Nope, still not the owner!",
"Sniff sniff... Nope, doesn't smell like you're the owner.",
"If you were a superhero, your power would definitely not be 'using owner only commands'.",
"Guess what? You just triggered the 'not the owner' alarm!",
"Oh no! Your 'not the owner' is showing.",
"You must be a distant relative of Sherlock Holmes with detective skills like that!",
"Spoiler alert: You're not the owner.",
"Trying to use an owner only command? That's a paddlin'.",
"Maybe in another life, you'll be the owner. But not today.",
"Do you also try to enter secret societies with a 'please' and a smile?",
"You must be a rebel, trying to use commands above your pay grade!",
"Did you think there was a secret handshake to access this command?",
"Plotting world domination? Start by owning a bot first."
];

/// An embed for when someone runs a privileged command that they do not have permission for.
pub fn permission_not_bot_staff() -> EmbedBuilder {
    let mut embed = EmbedBuilder::default();
    embed
        .title("You are not the bot owner!")
        .colour(crate::COLOUR_DANGER)
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
