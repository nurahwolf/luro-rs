use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::Interaction,
    channel::message::{
        component::{ActionRow, TextInput, TextInputStyle, Button, ButtonStyle},
        Component, MessageFlags,
    },
};
use twilight_util::builder::{embed::{EmbedAuthorBuilder, EmbedBuilder, ImageSource}, InteractionResponseDataBuilder};

use crate::{
    functions::{
        get_partial_member_avatar, interaction_context, parse_modal_data,
        parse_modal_field_required,
    },
    interactions::InteractionResponse,
    responses::embeds::invalid_heck,
    ACCENT_COLOUR, LuroContext,
};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "add", desc = "Add a heck", dm_permission = true)]
pub struct HeckAddCommand {}

impl HeckAddCommand {
    pub async fn run(&self, ctx: LuroContext, interaction: &Interaction) -> anyhow::Result<InteractionResponse> {

        Ok(heck_modal())
    }
}

/// Modal that asks the user to enter a reason for the kick.
///
/// This modal is only shown if the user has not specified a reason in the
/// initial command.
fn heck_modal() -> InteractionResponse {
    let components = vec![Component::ActionRow(ActionRow {
        components: vec![Component::TextInput(TextInput {
            custom_id: "heck-text".to_string(),
            label: "Enter your new heck below".to_string(),
            max_length: Some(2048),
            min_length: Some(20),
            placeholder: Some("<author> just gave <user> headpats!!".to_string()),
            required: Some(true),
            style: TextInputStyle::Paragraph,
            value: None,
        })],
    })];

    InteractionResponse::Modal {
        custom_id: "heck-add".to_string(),
        title: "Write your heck below!".to_string(),
        components,
    }
}

pub async fn handle_heck_model(
    interaction: Interaction,
) -> Result<InteractionResponse, anyhow::Error> {
    let (_, interaction_author, interaction_member) =
        interaction_context(&interaction, "heck add")?;
    let author_avatar = get_partial_member_avatar(
        interaction_member,
        &interaction.guild_id,
        interaction_author,
    );
    let data = parse_modal_data(&mut interaction.clone())?;
    let heck_text = parse_modal_field_required(&data, "heck-text")?;

    if !heck_text.contains("<user>") && !heck_text.contains("<author>") { return Ok(invalid_heck::response(true, true, heck_text)) };
    if !heck_text.contains("<user>") { return Ok(invalid_heck::response(true, false, heck_text)) };
    if !heck_text.contains("<author>") { return Ok(invalid_heck::response(false, true, heck_text)) };

    // Send a success message.
    let embed_author =
        EmbedAuthorBuilder::new(format!("Brand new heck by {}", interaction_author.name))
            .icon_url(ImageSource::url(author_avatar)?)
            .build();
    let embed = EmbedBuilder::new()
        .color(ACCENT_COLOUR)
        .description(heck_text)
        .author(embed_author);

    Ok(InteractionResponse::Embed {
        embeds: vec![embed.build()],
        components: None,
        ephemeral: true,
    })
}
