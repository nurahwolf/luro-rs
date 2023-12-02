use luro_framework::{CommandInteraction, CreateLuroCommand, ModalInteraction};
use luro_model::types::CommandResponse;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{channel::message::component::TextInputStyle, http::interaction::InteractionResponseType};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "embed", desc = "Send a simple embed")]
pub struct Embed {}

impl CreateLuroCommand for Embed {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        ctx.respond(|r| {
            r.title("Setup a modal below")
                .components(|components| {
                    components
                        .action_row(|row| {
                            row.text_input(|text| {
                                text.custom_id("modal-title")
                                    .label("Embed Title")
                                    .placeholder("hey guys, vsauce here")
                                    .style(TextInputStyle::Short)
                            })
                        })
                        .action_row(|row| {
                            row.text_input(|text| {
                                text.custom_id("embed-description")
                                    .label("Embed Description")
                                    .placeholder("I'm gonna write the longest shitpost ever...")
                            })
                        })
                })
                .custom_id("embed")
                .response_type(InteractionResponseType::Modal)
        })
        .await
    }

    async fn interaction_modal(ctx: ModalInteraction) -> anyhow::Result<CommandResponse> {
        let description = ctx.parse_field_required("embed-description")?;

        ctx.respond(|r| r.embed(|embed| embed.description(description).colour(2829617_u32)))
            .await
    }
}
