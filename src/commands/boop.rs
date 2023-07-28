use async_trait::async_trait;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::{command::Command, interaction::message_component::MessageComponentInteractionData},
    channel::message::component::{ActionRow, Button, ButtonStyle, Component}
};

use crate::responses::LuroSlash;

use super::LuroCommand;

pub fn commands() -> Vec<Command> {
    vec![BoopCommand::create_command().into()]
}

#[derive(CommandModel, CreateCommand, Default, Debug, PartialEq, Eq)]
#[command(name = "boop", desc = "Boop the Bot!")]
pub struct BoopCommand {}

#[async_trait]
impl LuroCommand for BoopCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
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

        ctx.content("Boop Count: 0".to_owned()).components(components).respond().await
    }

    async fn handle_button(self, ctx: LuroSlash, _data: MessageComponentInteractionData) -> anyhow::Result<()> {
        // Get message and parse number
        let message = ctx.interaction.message.clone().unwrap();

        let (_text, number) = message.content.split_at(12);

        let value_number = match number.parse::<i32>() {
            Ok(v) => v + 1,
            Err(_) => 0
        };

        ctx.content(format!("Boop Count: {}", value_number)).update().respond().await
    }
}

impl BoopCommand {}
