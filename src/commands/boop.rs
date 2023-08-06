use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::message_component::MessageComponentInteractionData,
    channel::message::component::{ActionRow, Button, ButtonStyle, Component}
};

use crate::{models::LuroResponse, LuroContext};

use crate::traits::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand, Default, Debug, PartialEq, Eq)]
#[command(name = "boop", desc = "Boop the Bot!")]
pub struct BoopCommand {}

#[async_trait]
impl LuroCommand for BoopCommand {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
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

        slash.content("Boop Count: 0".to_owned()).components(components);
        ctx.respond(&mut slash).await
    }

    async fn handle_component(
        _data: Box<MessageComponentInteractionData>,
        ctx: &LuroContext,
        slash: &mut LuroResponse
    ) -> anyhow::Result<()> {
        // Get message and parse number
        let message = slash.interaction.message.clone().unwrap();

        let (_text, number) = message.content.split_at(12);

        let value_number = match number.parse::<i32>() {
            Ok(v) => v + 1,
            Err(_) => 0
        };

        slash.content(format!("Boop Count: {}", value_number)).update();
        ctx.respond(slash).await
    }
}

impl BoopCommand {}
