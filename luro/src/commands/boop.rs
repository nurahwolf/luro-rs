use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::{command::Command, interaction::Interaction},
    channel::message::component::{ActionRow, Button, ButtonStyle, Component}
};

use crate::{interactions::InteractionResponse, LuroContext, SlashResponse};

pub fn commands() -> Vec<Command> {
    vec![BoopCommand::create_command().into()]
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "boop", desc = "Boop the Bot!")]
pub struct BoopCommand {}

impl BoopCommand {
    pub async fn run(_interaction: &Interaction, _ctx: &LuroContext) -> SlashResponse {
        let components = Vec::from([Component::ActionRow(ActionRow {
            components: Vec::from([Component::Button(Button {
                custom_id: Some(String::from("boop")),
                disabled: false,
                emoji: None,
                label: Some(String::from("Boop Me!")),
                style: ButtonStyle::Primary,
                url: None
            })])
        })]);

        Ok(InteractionResponse::ContentComponents {
            content: "Boop Count: 0".to_string(),
            components,
            ephemeral: false,
            deferred: false
        })
    }

    pub async fn button(interaction: &Interaction) -> SlashResponse {
        // Get message and parse number
        let message = interaction.message.clone().unwrap();

        let (_text, number) = message.content.split_at(12);

        let value_number = match number.parse::<i32>() {
            Ok(v) => v + 1,
            Err(_) => 0
        };

        Ok(InteractionResponse::Update {
            content: Some(format!("Boop Count: {}", value_number)),
            embeds: None,
            components: None,
            ephemeral: false
        })
    }
}
