use async_trait::async_trait;
use luro_framework::{command::ExecuteLuroCommand, CommandInteraction};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::channel::message::component::SelectMenuType;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "mass_assign", desc = "Mass assign a bunch of roles.")]
/// The name is slightly annoying on this one, its for the /owner commands subcommand, which is used for registering or deregistering commands globally.
pub struct MassAssign {}

#[async_trait]
impl ExecuteLuroCommand for MassAssign {
    async fn interaction_command(&self, ctx: CommandInteraction<()>) -> anyhow::Result<()> {
        ctx.respond(|response| {
            {
                response
                    .content("Select what roles should be matched. Select none to match all users without roles.")
                    .components(|components| {
                        components.action_row(|row| {
                            row.component(|component| {
                                component.select_menu(|menu| {
                                    menu.custom_id("mass-assign-selector")
                                        .kind(SelectMenuType::Role)
                                        .max_values(25)
                                        .min_values(0)
                                })
                            })
                        })
                    })
            }
            .ephemeral()
        })
        .await
    }
}
