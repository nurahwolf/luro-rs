use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{channel::message::component::TextInputStyle, http::interaction::InteractionResponseType};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};
use luro_model::database::drivers::LuroDatabaseDriver;

#[derive(CommandModel, CreateCommand, Default, Debug, PartialEq, Eq)]
#[command(
    name = "modify",
    desc = "Modify something sent by Luro, such as adding components and modifying embeds",
    dm_permission = false
)]
pub struct Modify {}

impl LuroCommand for Modify {
    /// Modal that asks the user to enter a reason for the kick.
    ///
    /// This modal is only shown if the user has not specified a reason in the
    /// initial command.
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let channel_id = ctx.interaction.channel.clone().unwrap().id;
        ctx.respond(|r| {
            r.title("Modify an Embed!")
                .custom_id("modify-embed")
                .response_type(InteractionResponseType::Modal)
                .components(|components| {
                    components
                        .action_row(|row| {
                            row.text_input(|input| {
                                input
                                    .custom_id("embed-title")
                                    .label("Embed Title")
                                    .placeholder("Hey kids, wanna see a dead body?")
                                    .style(TextInputStyle::Short)
                                    .required(false)
                            })
                        })
                        .action_row(|row| {
                            row.text_input(|input| {
                                input
                                    .custom_id("embed-description")
                                    .label("Embed Description")
                                    .placeholder(
                                        "Look upon thy field in which thine grow minth fucks, and see that it laythe barren.",
                                    )
                                    .required(false)
                            })
                        })
                        .action_row(|row| {
                            row.text_input(|input| {
                                input
                                    .custom_id("message-id")
                                    .label("Message ID")
                                    .placeholder("42069420")
                                    .style(TextInputStyle::Short)
                            })
                        })
                        .action_row(|row| {
                            row.text_input(|input| {
                                input
                                    .custom_id("channel-id")
                                    .label("Channel ID")
                                    .placeholder("42069420")
                                    .style(TextInputStyle::Short)
                                    .value(channel_id)
                            })
                        })
                })
                .response_type(InteractionResponseType::Modal)
        })
        .await
    }
}
