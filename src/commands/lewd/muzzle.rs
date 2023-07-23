use async_trait::async_trait;
use rand::Rng;
use twilight_gateway::MessageSender;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::application::interaction::Interaction;

use crate::{responses::LuroResponseV2, LuroContext, SlashResponse};

use super::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "muzzle", desc = "Put a muzzle on a user")]
pub struct MuzzleCommand {
    /// The user to muzzle.
    user: ResolvedUser
}

#[async_trait]
impl LuroCommand for MuzzleCommand {
    async fn run_command(self, interaction: Interaction, _ctx: LuroContext, _shard: MessageSender) -> SlashResponse {
        let responses = ["<user> just got muzzled for a few seconds!!",
        "<user> just got slapped on the muzzle and told to hush.",
        "<user> just got spanked and told to hush up immediately!",
        "<user> was forced on their knees and told to beg to be allowed to speak again.",
        "<user> just had duct tape wrapped around their mouth!",
        "A ballgag was stuffed into <user>'s mouth!",
        "<user> was very naughty.",
        "<user> deserves punishment for speaking when they should not.",
        "<user> was knotted on both ends in order to get them to shut up."];

        let choice = rand::thread_rng().gen_range(0..responses.len());
        let response_text = responses.get(choice).unwrap().replace("<user>", format!("<@{}>",self.user.resolved.id).as_str());
        let response = LuroResponseV2::new("lewd muzzle".to_owned(), &interaction);
        Ok(response.content(response_text).legacy_response(false))
    }
}
