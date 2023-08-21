use anyhow::anyhow;
use anyhow::Error;

use luro_builder::embed::EmbedBuilder;
use twilight_interactions::command::CommandModel;

use twilight_model::{
    application::interaction::{
        application_command::CommandData, message_component::MessageComponentInteractionData, modal::ModalInteractionData,
        Interaction
    },
    channel::Channel,
    guild::{PartialMember, Permissions},
    id::{marker::GuildMarker, Id},
    user::User
};

use crate::interaction::LuroSlash;
use crate::LuroFramework;
use luro_model::database::drivers::LuroDatabaseDriver;

/// Add some custom functionality around [CommandModel]
pub trait LuroCommand: CommandModel {
    /// Create a command that can be executed with Twilight
    // fn commands() -> Vec<Command> {
    //     vec![Self::create_command().into()]
    // }

    /// Create a new command and get it's data from the interaction
    async fn new(data: CommandData) -> anyhow::Result<Self> {
        match Self::from_interaction(data.into()) {
            Ok(ok) => Ok(ok),
            Err(why) => Err(anyhow!(
                "Got interaction data, but failed to parse it to the command type specified: {why}"
            ))
        }
    }

    /// Run the command / command group
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        ctx.not_implemented_response().await
    }

    /// Handle a component interaction. This could be a button or other form of interaciton
    async fn handle_component<D: LuroDatabaseDriver>(
        self,
        _data: Box<MessageComponentInteractionData>,
        ctx: LuroSlash<D>
    ) -> anyhow::Result<()> {
        ctx.not_implemented_response().await
    }

    /// Create and respond to a button interaction
    async fn handle_model<D: LuroDatabaseDriver>(_data: ModalInteractionData, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        ctx.not_implemented_response().await
    }

    /// The default permissions a user needs to run this command
    fn default_permissions() -> Permissions {
        Permissions::all()
    }

    /// A function that takes a borred interaction, and returns a borred reference to interaction.channel and a user who invoked the interaction. Additionally it calls a debug to print where the command was executed in the logs
    fn interaction_context<'a>(
        &self,
        interaction: &'a Interaction,
        command_name: &str
    ) -> anyhow::Result<(&'a Channel, &'a User, Option<&'a PartialMember>)> {
        let invoked_channel = interaction
            .channel
            .as_ref()
            .ok_or_else(|| Error::msg("Unable to get the channel this interaction was ran in"))?;
        let interaction_member = interaction.member.as_ref();
        let interaction_author = match interaction.member.as_ref() {
            Some(member) => member
                .user
                .as_ref()
                .ok_or_else(|| Error::msg("Unable to find the user that executed this command"))?,
            None => interaction
                .user
                .as_ref()
                .ok_or_else(|| Error::msg("Unable to find the user that executed this command"))?
        };

        match &invoked_channel.name {
            Some(channel_name) => tracing::debug!(
                "'{}' interaction in channel {} by {}",
                command_name,
                channel_name,
                interaction_author.name
            ),
            None => tracing::debug!("'{}' interaction by {}", command_name, interaction_author.name)
        };

        Ok((invoked_channel, interaction_author, interaction_member))
    }

    // fn parse_component_data(
    //     self,
    //     interaction: &mut Interaction
    // ) -> Result<Box<MessageComponentInteractionData>, anyhow::Error> {
    //     match mem::take(&mut interaction.data) {
    //         Some(InteractionData::MessageComponent(data)) => Ok(data),
    //         _ => bail!("unable to parse modal data, received unknown data type")
    //     }
    // }

    /// Create a default embed which has the guild's accent colour if available, otherwise falls back to Luro's accent colour
    async fn default_embed(&self, ctx: &LuroFramework, guild_id: Option<Id<GuildMarker>>) -> EmbedBuilder {
        ctx.default_embed(&guild_id).await
    }

    /// Attempts to get the guild's accent colour, else falls back to getting the hardcoded accent colour
    async fn accent_colour(&self, ctx: &LuroFramework, guild_id: Option<Id<GuildMarker>>) -> u32 {
        ctx.accent_colour(&guild_id).await
    }

    // TODO: WTF is this?
    fn assemble_user_avatar(&self, user: &User) -> String {
        let user_id = user.id;
        user.avatar.map_or_else(
            || format!("https://cdn.discordapp.com/embed/avatars/{}.png", user.discriminator % 5),
            |avatar| format!("https://cdn.discordapp.com/avatars/{user_id}/{avatar}.png")
        )
    }
}
