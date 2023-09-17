use async_trait::async_trait;
use luro_framework::{command::LuroCommandTrait, Framework, InteractionCommand, LuroInteraction};
use luro_model::database_driver::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::channel::message::component::SelectMenuType;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "mass_assign", desc = "Mass assign a bunch of roles.")]
/// The name is slightly annoying on this one, its for the /owner commands subcommand, which is used for registering or deregistering commands globally.
pub struct MassAssign {}

#[async_trait]
impl LuroCommandTrait for MassAssign {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        interaction
            .respond(&ctx, |response| {
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
