use async_trait::async_trait;
use twilight_gateway::MessageSender;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::{command::Command, interaction::Interaction},
    channel::message::component::{ActionRow, Button, ButtonStyle, Component}
};

use crate::{interactions::InteractionResponse, LuroContext, SlashResponse};

use super::LuroCommand;

pub fn commands() -> Vec<Command> {
    vec![BoopCommand::create_command().into()]
}

#[derive(CommandModel, CreateCommand, Default, Debug, PartialEq, Eq)]
#[command(name = "boop", desc = "Boop the Bot!")]
pub struct BoopCommand {}

#[async_trait]
impl LuroCommand for BoopCommand {
    async fn run_command(self, _interaction: Interaction, _ctx: LuroContext, _shard: MessageSender) -> SlashResponse {
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
            luro_response: Default::default()
        })
    }

    async fn handle_button(self, interaction: Interaction) -> SlashResponse {
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

impl BoopCommand {}
