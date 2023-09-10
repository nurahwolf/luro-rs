use luro_builder::embed::EmbedBuilder;
use luro_model::database::drivers::LuroDatabaseDriver;
use luro_model::user::actions::UserActions;
use luro_model::user::actions_type::UserActionType;
use rand::seq::SliceRandom;
use tracing::warn;
use twilight_model::id::marker::{GuildMarker, UserMarker};
use twilight_model::id::Id;

use crate::interaction::LuroSlash;
use crate::COLOUR_DANGER;

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

impl<D: LuroDatabaseDriver> LuroSlash<D> {
    pub async fn not_owner_response(
        &self,
        user_id: &Id<UserMarker>,
        guild_id: &Option<Id<GuildMarker>>,
        command_name: impl Into<String>
    ) -> anyhow::Result<()> {
        let command = command_name.into();
        {
            let mut user_data = self.framework.database.get_user(user_id, false).await?;
            user_data.moderation_actions.push(UserActions {
                action_type: vec![UserActionType::PrivilegeEscalation],
                guild_id: *guild_id,
                reason: Some(format!("Attempted to run the {} command", &command)),
                responsible_user: *user_id
            });
            self.framework.database.save_user(user_id, &user_data).await?;
        }
        self.respond(|r| r.add_embed(not_owner_embed(user_id, &command))).await
    }
}

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
fn not_owner_embed(user_id: &Id<UserMarker>, command_name: &String) -> EmbedBuilder {
    warn!("User {user_id} attempted to run the command {command_name} without being in my list of authorised users...");
    let mut embed = EmbedBuilder::default();
    let mut rng = rand::thread_rng();
    let insult = INSULTS.choose(&mut rng).unwrap_or(&INSULTS[0]);

    embed
        .title("You are not the bot owner!")
        .colour(COLOUR_DANGER)
        .description(insult)
        .footer(|f| f.text("FYI, I'm reporting you to Nurah."));
    embed
}
