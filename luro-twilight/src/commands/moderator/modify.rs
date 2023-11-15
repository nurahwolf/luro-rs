use luro_framework::{CommandInteraction, LuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{channel::message::component::TextInputStyle, http::interaction::InteractionResponseType};

#[derive(CommandModel, CreateCommand, Default, Debug, PartialEq, Eq)]
#[command(
    name = "modify",
    desc = "ADMINISTRATOR: Modify something sent by Luro, such as adding components and modifying embeds",
    dm_permission = false
)]
pub struct Modify {}

impl LuroCommand for Modify {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
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
                                    .placeholder("Look upon thy field in which thine grow minth fucks, and see that it laythe barren.")
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
                                    .value(ctx.channel.id)
                            })
                        })
                })
                .response_type(InteractionResponseType::Modal)
        })
        .await
    }
}
