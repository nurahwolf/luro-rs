use crate::framework::LuroFramework;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::{command::Command, interaction::Interaction},
    channel::message::component::{ActionRow, Button, ButtonStyle, Component},
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};

pub fn commands() -> Vec<Command> {
    vec![BoopCommand::create_command().into()]
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "boop", desc = "Boop the Bot!")]
pub struct BoopCommand {}

impl BoopCommand {
    pub async fn run() -> anyhow::Result<crate::interactions::InteractionResponse> {
        let components = Some(Vec::from([Component::ActionRow(ActionRow {
            components: Vec::from([Component::Button(Button {
                custom_id: Some(String::from("boop")),
                disabled: false,
                emoji: None,
                label: Some(String::from("Boop Me!")),
                style: ButtonStyle::Primary,
                url: None,
            })]),
        })]));

        Ok(crate::interactions::InteractionResponse::Text {
            content: "Boop Count: 0".to_string(),
            components,
            ephemeral: true,
        })
    }

    pub async fn button(
        ctx: &LuroFramework,
        interaction: &Interaction,
    ) -> anyhow::Result<crate::interactions::InteractionResponse> {
        // Get message and parse number
        let message = interaction.message.clone().unwrap();

        let (_text, number) = message.content.split_at(12);

        let value_number = match number.parse::<i32>() {
            Ok(v) => v + 1,
            Err(_) => 0,
        };

        // Update message as interaction response
        let response = InteractionResponse {
            kind: InteractionResponseType::UpdateMessage,
            data: Some(InteractionResponseData {
                content: Some(format!("Boop Count: {}", value_number)),
                ..Default::default()
            }),
        };

        match ctx
            .interaction_client()
            .create_response(interaction.id, &interaction.token, &response)
            .await
        {
            Ok(ok) => ok,
            Err(_) => todo!(),
        };

        Ok(crate::interactions::InteractionResponse::Raw {
            kind: InteractionResponseType::UpdateMessage,
            data: Some(InteractionResponseData {
                content: Some(format!("Boop Count: {}", value_number)),
                ..Default::default()
            }),
        })
    }
}
