use luro_framework::{CommandInteraction, ComponentInteraction, CreateLuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::channel::message::component::{ActionRow, Button, ButtonStyle, Component};

#[derive(CommandModel, CreateCommand, Default, Debug, PartialEq, Eq)]
#[command(name = "boop", desc = "Boop the Bot!")]
pub struct Boop {}

impl CreateLuroCommand for Boop {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let components = Vec::from([Component::ActionRow(ActionRow {
            components: Vec::from([Component::Button(Button {
                custom_id: Some(String::from("boop")),
                disabled: false,
                emoji: None,
                label: Some(String::from("Boop Me!")),
                style: ButtonStyle::Primary,
                url: None,
            })]),
        })]);

        ctx.respond(|r| r.content("Boop Count: 0").add_components(components)).await
    }

    async fn interaction_component(
        self,
        ctx: ComponentInteraction,
        _: twilight_model::application::interaction::Interaction,
    ) -> anyhow::Result<()> {
        // Get message and parse number
        let message = ctx.message.clone();

        let (_text, number) = message.content.split_at(12);

        let value_number = match number.parse::<i32>() {
            Ok(v) => v + 1,
            Err(_) => 0,
        };

        ctx.respond(|r| r.content(format!("Boop Count: {}", value_number)).update()).await
    }
}
