use anyhow::anyhow;
use std::collections::HashMap;
use twilight_util::builder::embed::EmbedBuilder;

use twilight_interactions::command::CreateCommand;
use twilight_model::{
    application::{command::Command, interaction::application_command::InteractionMember},
    guild::Member,
    id::{marker::GuildMarker, Id},
    user::CurrentUser
};

use self::{
    about::AboutCommand, base64::Base64Commands, boop::BoopCommand, count::CountCommand, heck::HeckCommands,
    hello::HelloCommand, lewd::LewdCommands, moderator::ModeratorCommands, music::MusicCommands, owner::OwnerCommands,
    say::SayCommand, user::UserCommands
};

use anyhow::bail;
use tracing::warn;

use twilight_model::application::interaction::{Interaction, InteractionData};

use crate::{
    functions::{accent_colour, default_embed, get_user_avatar},
    responses::LuroSlash,
    LuroContext
};

pub mod about;
pub mod base64;
pub mod boop;
pub mod count;
pub mod heck;
pub mod hello;
pub mod lewd;
pub mod moderator;
pub mod music;
pub mod owner;
pub mod say;
pub mod user;

#[derive(Default)]
pub struct Commands {
    /// Commands that are available to be registered within guilds
    pub guild_commands: HashMap<&'static str, Command>,
    /// Commands that are available to be registered globally
    pub global_commands: HashMap<&'static str, Command>
}

impl Commands {
    /// Return a default set of commands to register
    pub fn default_commands() -> Self {
        // Create the hashmaps
        let mut init = Self {
            guild_commands: Default::default(),
            global_commands: Default::default()
        };

        // Add some default commands
        init.global_commands.insert("hello", HelloCommand::create_command().into());
        init.global_commands.insert("count", CountCommand::create_command().into());
        init.global_commands.insert("say", SayCommand::create_command().into());
        init.global_commands.insert("mod", ModeratorCommands::create_command().into());
        init.global_commands.insert("music", MusicCommands::create_command().into());
        init.global_commands.insert("boop", BoopCommand::create_command().into());
        init.global_commands.insert("heck", HeckCommands::create_command().into());
        init.global_commands.insert("owner", OwnerCommands::create_command().into());
        init.global_commands.insert("about", AboutCommand::create_command().into());
        init.global_commands.insert("user", UserCommands::create_command().into());
        init.global_commands.insert("lewd", LewdCommands::create_command().into());
        init.global_commands.insert("base64", Base64Commands::create_command().into());

        // Return our initialised commands
        init
    }
}

impl LuroSlash {
    /// Handle incoming command interaction.
    pub async fn handle_command(self) -> anyhow::Result<()> {
        let data = match self.interaction.data.clone() {
            Some(InteractionData::ApplicationCommand(data)) => *data,
            _ => bail!("expected application command data")
        };

        match data.name.as_str() {
            "about" => AboutCommand::new(data).await?.run_command(self).await,
            "say" => SayCommand::new(data).await?.run_command(self).await,
            "user" => UserCommands::new(data).await?.run_commands(self).await,
            "hello" => HelloCommand::new(data).await?.run_command(self).await,
            "count" => CountCommand::new(data).await?.run_command(self).await,
            "mod" => ModeratorCommands::new(data).await?.run_commands(self).await,
            "music" => MusicCommands::new(data).await?.run_commands(self).await,
            "boop" => BoopCommand::new(data).await?.run_command(self).await,
            "owner" => OwnerCommands::new(data).await?.run_commands(self).await,
            "heck" => HeckCommands::new(data).await?.run_commands(self).await,
            "lewd" => LewdCommands::new(data).await?.run_commands(self).await,
            "base64" => Base64Commands::new(data).await?.run_commands(self).await,
            _ => self.unknown_command_response().await
        }
    }
}

use anyhow::{Context, Error};
use async_trait::async_trait;
use std::mem;
use twilight_interactions::command::CommandModel;
use twilight_model::{
    application::interaction::{
        application_command::CommandData, message_component::MessageComponentInteractionData, modal::ModalInteractionData
    },
    channel::Channel,
    guild::{PartialMember, Permissions},
    user::User
};

/// Add some custom functionality around [CommandModel]
#[async_trait]
pub trait LuroCommand: CommandModel {
    /// Create a new command and get it's data from the interaction
    async fn new(data: CommandData) -> anyhow::Result<Self> {
        Self::from_interaction(data.into()).context("failed to parse command data")
    }

    /// Run the command
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        ctx.not_implemented_response().await
    }

    /// Run a command group
    async fn run_commands(self, ctx: LuroSlash) -> anyhow::Result<()> {
        ctx.not_implemented_response().await
    }

    /// Handle a component interaction
    async fn handle_component(self, ctx: LuroSlash) -> anyhow::Result<()> {
        ctx.not_implemented_response().await
    }

    /// Create and respond to a button interaction
    async fn handle_button(self, ctx: LuroSlash) -> anyhow::Result<()> {
        ctx.not_implemented_response().await
    }

    /// Create and respond to a button interaction
    async fn handle_model(self, ctx: LuroSlash) -> anyhow::Result<()> {
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

    fn parse_component_data(self, interaction: &mut Interaction) -> Result<MessageComponentInteractionData, anyhow::Error> {
        match mem::take(&mut interaction.data) {
            Some(InteractionData::MessageComponent(data)) => Ok(data),
            _ => bail!("unable to parse modal data, received unknown data type")
        }
    }

    /// Parse incoming [`ModalSubmit`] interaction and return the inner data.
    ///
    /// This takes a mutable [`Interaction`] since the inner [`ModalInteractionData`]
    /// is replaced with [`None`] to avoid useless clones.
    ///
    /// [`ModalSubmit`]: twilight_model::application::interaction::InteractionType::ModalSubmit
    /// [`ModalInteractionData`]: twilight_model::application::interaction::modal::ModalInteractionData
    fn parse_modal_data(&self, interaction: &mut Interaction) -> Result<ModalInteractionData, anyhow::Error> {
        match mem::take(&mut interaction.data) {
            Some(InteractionData::ModalSubmit(data)) => Ok(data),
            _ => bail!("unable to parse modal data, received unknown data type")
        }
    }

    /// Parse a field from [`ModalInteractionData`].
    ///
    /// This function try to find a field with the given name in the modal data and
    /// return its value as a string.
    fn parse_modal_field<'a>(&self, data: &'a ModalInteractionData, name: &str) -> Result<Option<&'a str>, anyhow::Error> {
        let mut components = data.components.iter().flat_map(|c| &c.components);

        match components.find(|c| &*c.custom_id == name) {
            Some(component) => Ok(component.value.as_deref()),
            None => bail!("missing modal field: {}", name)
        }
    }

    /// Parse a required field from [`ModalInteractionData`].
    ///
    /// This function is the same as [`parse_modal_field`] but returns an error if
    /// the field value is [`None`].
    fn parse_modal_field_required<'a>(&self, data: &'a ModalInteractionData, name: &str) -> Result<&'a str, anyhow::Error> {
        let value = self.parse_modal_field(data, name)?;

        value.ok_or_else(|| anyhow!("required modal field is empty: {}", name))
    }

    /// Create a default embed which has the guild's accent colour if available, otherwise falls back to Luro's accent colour
    fn default_embed(&self, ctx: &LuroContext, guild_id: Option<Id<GuildMarker>>) -> EmbedBuilder {
        default_embed(ctx, &guild_id)
    }

    /// Attempts to get the guild's accent colour, else falls back to getting the hardcoded accent colour
    fn accent_colour(&self, ctx: &LuroContext, guild_id: Option<Id<GuildMarker>>) -> u32 {
        accent_colour(ctx, &guild_id)
    }

    fn assemble_user_avatar(&self, user: &User) -> String {
        let user_id = user.id;
        user.avatar.map_or_else(
            || format!("https://cdn.discordapp.com/embed/avatars/{}.png", user.discriminator % 5),
            |avatar| format!("https://cdn.discordapp.com/avatars/{user_id}/{avatar}.png")
        )
    }

    /// Return the user's avatar fromH
    fn get_partial_member_avatar(
        &self,
        member: Option<&PartialMember>,
        guild_id: &Option<Id<GuildMarker>>,
        user: &User
    ) -> String {
        let user_id = user.id;

        if let Some(member) = member && let Some(guild_id) = guild_id && let Some(member_avatar) = member.avatar {
            match member_avatar.is_animated() {
                true => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.gif"),
                false => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.png"),
            }
        };

        self.get_user_avatar(user)
    }

    /// Return the user's avatar fromH
    fn get_interaction_member_avatar(
        &self,
        member: Option<InteractionMember>,
        guild_id: &Option<Id<GuildMarker>>,
        user: &User
    ) -> String {
        let user_id = user.id;

        if let Some(member) = member && let Some(guild_id) = guild_id && let Some(member_avatar) = member.avatar {
            match member_avatar.is_animated() {
                true => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.gif"),
                false => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.png"),
            }
        };

        self.get_user_avatar(user)
    }

    /// Return a string that is a link to the member's banner, falling back to a user banner if it present. Returns [None] if the user does not have a banner at all.
    fn get_member_banner(&self, _member: &Member, _guild_id: Id<GuildMarker>, user: &User) -> Option<String> {
        let _user_id = user.id;

        // TODO: Looks like this is not possible currently, due to Twilight not having a guild_banner object.

        // if let Some(member) = member && let Some(guild_id) = guild_id && let Some(member_avatar) = member. {
        //     match member_avatar.is_animated() {
        //         true => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.gif"),
        //         false => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.png"),
        //     }
        // };

        self.get_user_banner(user)
    }

    /// Return a string that is a link to the user's banner, or [None] if they don't have one
    fn get_user_banner(&self, user: &User) -> Option<String> {
        let user_id = user.id;

        if let Some(banner) = user.banner {
            match banner.is_animated() {
                true => Some(format!("https://cdn.discordapp.com/banner/{user_id}/{banner}.gif")),
                false => Some(format!("https://cdn.discordapp.com/avatars/{user_id}/{banner}.png"))
            }
        } else {
            None
        }
    }

    /// Return a string that is a link to the member's avatar, falling back to user avatar if it does not exist
    fn get_member_avatar(&self, member: Option<&Member>, guild_id: &Option<Id<GuildMarker>>, user: &User) -> String {
        let user_id = user.id;

        if let Some(member) = member && let Some(guild_id) = guild_id && let Some(member_avatar) = member.avatar {
            match member_avatar.is_animated() {
                true => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.gif"),
                false => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.png"),
            }
        };

        self.get_user_avatar(user)
    }

    /// Return a string that is a link to the user's avatar
    fn get_user_avatar(&self, user: &User) -> String {
        get_user_avatar(user)
    }

    /// Return a string that is a link to the user's avatar
    fn get_currentuser_avatar(&self, currentuser: &CurrentUser) -> String {
        let user_id = currentuser.id;

        if let Some(user_avatar) = currentuser.avatar {
            match user_avatar.is_animated() {
                true => return format!("https://cdn.discordapp.com/avatars/{user_id}/{user_avatar}.gif"),
                false => return format!("https://cdn.discordapp.com/avatars/{user_id}/{user_avatar}.png")
            }
        };

        let modulo = currentuser.discriminator % 5;
        format!("https://cdn.discordapp.com/embed/avatars/{modulo}.png")
    }
}
