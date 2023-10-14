use luro_framework::{command::ExecuteLuroCommand, CommandInteraction};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::GenericMarker, Id};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "commands", desc = "By default all this does is deregister commands in a guild")]
/// The name is slightly annoying on this one, its for the /owner commands subcommand, which is used for registering or deregistering commands globally.
pub struct Commands {
    /// The guild to reregister
    guild: Id<GenericMarker>,
}

impl ExecuteLuroCommand for Commands {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let application = ctx.twilight_client.current_user_application().await?.model().await?;
        let client = ctx.twilight_client.interaction(application.id);

        client.set_guild_commands(self.guild.cast(), &[]).await?;
        ctx.respond(|r| r.content(format!("Commands set to null in guild <#{}>", self.guild)).ephemeral())
            .await
    }
}
