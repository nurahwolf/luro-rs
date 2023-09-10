use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::GenericMarker, Id};

use crate::interaction::LuroSlash;
use luro_model::database_driver::LuroDatabaseDriver;

use crate::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "commands", desc = "By default all this does is deregister commands in a guild")]
/// The name is slightly annoying on this one, its for the /owner commands subcommand, which is used for registering or deregistering commands globally.
pub struct OwnerCommandsCommand {
    /// The guild to reregister
    guild: Id<GenericMarker>,
}

impl LuroCommand for OwnerCommandsCommand {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let application = ctx
            .framework
            .twilight_client
            .current_user_application()
            .await?
            .model()
            .await?;
        let client = ctx.framework.twilight_client.interaction(application.id);

        client.set_guild_commands(Id::new(self.guild.get()), &[]).await?;
        ctx.respond(|r| {
            r.content(format!("Commands set to null in guild <#{}>", self.guild))
                .ephemeral()
        })
        .await
    }
}
