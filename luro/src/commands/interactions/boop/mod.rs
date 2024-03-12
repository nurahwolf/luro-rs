use crate::models::interaction::{InteractionContext, InteractionResult};

#[derive(
    twilight_interactions::command::CommandModel, twilight_interactions::command::CreateCommand,
)]
#[command(name = "boop", desc = "Boop the Bot!")]
pub struct Boop {}

impl crate::models::CreateCommand for Boop {
    async fn handle_command(self, framework: &mut InteractionContext) -> InteractionResult<()> {
        framework.respond(|r| {
            r.components(|c| {
                c.action_row(|a_r| a_r.button(|btn| btn.custom_id("boop").label("Boop Me!")))
            })
            .content("Boop Count: 0")
        })
        .await
    }

    async fn handle_component(framework: &mut InteractionContext) -> InteractionResult<()> {
        // Get message and parse number
        let message = framework.compontent_message()?;

        let (_text, number) = message.content.split_at(12);

        let value_number = match number.parse::<i32>() {
            Ok(v) => v + 1,
            Err(_) => 0,
        };

        framework
            .respond(|r| r.content(format!("Boop Count: {}", value_number)).update())
            .await
    }
}
