use async_trait::async_trait;
use luro_framework::{command::LuroCommandTrait, Framework, InteractionCommand, LuroInteraction};
use luro_model::database_driver::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{channel::message::component::TextInputStyle, http::interaction::InteractionResponseType};

#[derive(CommandModel, CreateCommand, Default, Debug, PartialEq, Eq)]
#[command(
    name = "modify",
    desc = "ADMINISTRATOR: Modify something sent by Luro, such as adding components and modifying embeds",
    dm_permission = false
)]
pub struct Modify {}

#[async_trait]
impl LuroCommandTrait for Modify {
    /// Modal that asks the user to enter a reason for the kick.
    ///
    /// This modal is only shown if the user has not specified a reason in the
    /// initial command.
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let channel_id = interaction.channel.id;
        interaction
            .respond(&ctx, |r| {
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
