
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::message_component::MessageComponentInteractionData,
    channel::message::component::{ActionRow, Button, ButtonStyle, Component}
};

use crate::{slash::Slash, interaction::LuroSlash};

use crate::traits::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand, Default, Debug, PartialEq, Eq)]
#[command(name = "boop", desc = "Boop the Bot!")]
pub struct BoopCommand {}


impl LuroCommand for BoopCommand {
    async fn run_command(self, mut ctx: Slash) -> anyhow::Result<()> {
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

    async fn handle_component(self, _data: Box<MessageComponentInteractionData>, ctx: LuroSlash) -> anyhow::Result<()> {
        // Get message and parse number
        let message = ctx.interaction.message.clone().unwrap();

        let (_text, number) = message.content.split_at(12);

        let value_number = match number.parse::<i32>() {
            Ok(v) => v + 1,
            Err(_) => 0
        };

        ctx.respond(|r|r.content(format!("Boop Count: {}", value_number)).update()).await
    }
}

impl BoopCommand {}
