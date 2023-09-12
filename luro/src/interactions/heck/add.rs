use luro_model::database_driver::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::http::interaction::InteractionResponseType;

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

#[derive(CommandModel, CreateCommand, Default, Debug, PartialEq, Eq)]
#[command(name = "add", desc = "Add a heck", dm_permission = true)]
pub struct HeckAddCommand {}

impl LuroCommand for HeckAddCommand {
    /// Modal that asks the user to enter a reason for the kick.
    ///
    /// This modal is only shown if the user has not specified a reason in the
    /// initial command.
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        ctx.respond(|r| {
            r.title("Write your heck below!")
                .custom_id("heck-add")
                .components(|components| {
                    components.action_row(|row| {
                        row.text_input(|input| {
                            input
                                .custom_id("heck-text")
                                .label("Enter your new heck below")
                                .min_length(20)
                                .placeholder("<author> just gave <user> headpats!!")
                        })
                    })
                })
                .response_type(InteractionResponseType::Modal)
        })
        .await
    }
}
