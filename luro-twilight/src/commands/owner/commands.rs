use async_trait::async_trait;
use luro_framework::{command::LuroCommandTrait, Framework, InteractionCommand, LuroInteraction};
use luro_model::database_driver::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::GenericMarker, Id};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "commands", desc = "By default all this does is deregister commands in a guild")]
/// The name is slightly annoying on this one, its for the /owner commands subcommand, which is used for registering or deregistering commands globally.
pub struct Commands {
    /// The guild to reregister
    guild: Id<GenericMarker>,
}

#[async_trait]
impl LuroCommandTrait for Commands {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework<D>,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        let application = ctx.twilight_client.current_user_application().await?.model().await?;
        let client = ctx.twilight_client.interaction(application.id);

        client.set_guild_commands(Id::new(data.guild.get()), &[]).await?;
        interaction
            .respond(&ctx, |r| {
                r.content(format!("Commands set to null in guild <#{}>", data.guild))
                    .ephemeral()
            })
            .await
    }
}
