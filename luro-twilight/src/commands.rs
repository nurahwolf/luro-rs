use luro_framework::command::LuroCommand;
use luro_framework::responses::SimpleResponse;
use luro_framework::{Framework, InteractionCommand, InteractionComponent, InteractionModal};
use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_interactions::command::{ApplicationCommandData, CreateCommand};

use self::say::Say;

mod say;

/// Handle incoming command interaction.
pub async fn handle_command<D: LuroDatabaseDriver>(
    framework: Framework<D>,
    interaction: InteractionCommand
) -> anyhow::Result<()> {
    let data = interaction.data.clone();
    match data.name.as_str() {
        "say" => Say::new(data)?.interaction_command(framework, interaction).await,
        name => SimpleResponse::UnknownCommand(name).respond(framework, interaction).await
    }
}

/// Handle incoming component interaction
///
/// SAFETY: There is an unwrap here, but the type is always present on MessageComponent
/// which is the only type this function is called on
pub async fn handle_component<D: LuroDatabaseDriver>(
    _ctx: Framework<D>,
    _interaction: InteractionComponent
) -> anyhow::Result<()> {
    // match interaction.data.custom_id.as_str() {
    //     name => {
    //         warn!(name = name, "received unknown component");
    //         // self.unknown_command_response_named(name).await
    //         Ok(())
    //     }
    // }
    Ok(())
}

/// Handle incoming modal interaction
pub async fn handle_modal<D: LuroDatabaseDriver>(_ctx: Framework<D>, _interaction: InteractionModal) -> anyhow::Result<()> {
    // match interaction.data.custom_id.as_str() {
    //     name => {
    //         warn!(name = name, "received unknown component");
    //         // ctx.unknown_command_response_named(name).await
    //         Ok(())
    //     }
    // }
    Ok(())
}

/// Handle incoming autocomplete
pub async fn handle_autocomplete<D: LuroDatabaseDriver>(
    _ctx: Framework<D>,
    _interaction: InteractionCommand
) -> anyhow::Result<()> {
    Ok(())
}

pub fn default_global_commands() -> Vec<ApplicationCommandData> {
    vec![Say::create_command()]
}
