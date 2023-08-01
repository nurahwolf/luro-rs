use anyhow::{anyhow, bail};
use anyhow::{Context, Error};
use async_trait::async_trait;

use std::mem;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::application::command::Command;
use twilight_model::application::interaction::application_command::InteractionMember;
use twilight_model::application::interaction::InteractionData;
use twilight_model::guild::Member;
use twilight_model::user::CurrentUser;
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
use twilight_util::builder::embed::EmbedBuilder;

use crate::{models::LuroSlash, LuroContext};

/// Add some custom functionality around [CommandModel]
#[async_trait]
pub trait LuroCommand: CommandModel + CreateCommand {
    /// Create a command that can be executed with Twilight
    fn commands() -> Vec<Command> {
        vec![Self::create_command().into()]
    }

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
    async fn handle_component(self, ctx: LuroSlash, _data: MessageComponentInteractionData) -> anyhow::Result<()> {
        ctx.not_implemented_response().await
    }

    /// Create and respond to a button interaction
    async fn handle_button(self, ctx: LuroSlash, _data: MessageComponentInteractionData) -> anyhow::Result<()> {
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
        ctx.default_embed(&guild_id)
    }

    /// Attempts to get the guild's accent colour, else falls back to getting the hardcoded accent colour
    fn accent_colour(&self, ctx: &LuroContext, guild_id: Option<Id<GuildMarker>>) -> u32 {
        ctx.accent_colour(&guild_id)
    }

    fn assemble_user_avatar(&self, user: &User) -> String {
        let user_id = user.id;
        user.avatar.map_or_else(
            || format!("https://cdn.discordapp.com/embed/avatars/{}.png", user.discriminator % 5),
            |avatar| format!("https://cdn.discordapp.com/avatars/{user_id}/{avatar}.png")
        )
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
        crate::functions::get_user_avatar(user)
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

    /// Get and return useful information about the interaction author
    /// Returns the user, their avatar and a nicely formatted name
    fn get_interaction_author<'a>(&'a self, interaction: &'a Interaction) -> anyhow::Result<(&User, String, &String)> {
        Ok(match interaction.member {
            Some(ref member) => {
                let user = match &member.user {
                    Some(user) => user,
                    None => return Err(Error::msg("Expected user object within member"))
                };
                (
                    user,
                    self.get_partial_member_avatar(member, &interaction.guild_id, user),
                    match &member.nick {
                        Some(nick) => nick,
                        None => &user.name
                    }
                )
            }
            None => match interaction.user {
                Some(ref user) => (user, self.get_user_avatar(user), &user.name),
                None => return Err(Error::msg("No interaction member or user present"))
            }
        })
    }

    /// Get a specified user, else fall back to the interaction author
    /// Returns the user, their avatar and a nicely formatted name
    fn get_specified_user_or_author<'a>(
        &'a self,
        specified_user: &'a Option<ResolvedUser>,
        interaction: &'a Interaction
    ) -> anyhow::Result<(&User, String, &String)> {
        Ok(match specified_user {
            Some(user_defined) => (
                &user_defined.resolved,
                self.get_interaction_member_avatar(&user_defined.member, &interaction.guild_id, &user_defined.resolved),
                match user_defined.member {
                    Some(ref member) => match &member.nick {
                        Some(nick) => nick,
                        None => &user_defined.resolved.name
                    },
                    None => &user_defined.resolved.name
                }
            ),
            None => self.get_interaction_author(interaction)?
        })
    }

    /// Returns the avatar of an [InteractionMember]!
    fn get_interaction_member_avatar(
        &self,
        member: &Option<InteractionMember>,
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

    /// Returns the avatar of an [PartialMember]!
    fn get_partial_member_avatar(&self, member: &PartialMember, guild_id: &Option<Id<GuildMarker>>, user: &User) -> String {
        let user_id = user.id;

        if let Some(guild_id) = guild_id && let Some(member_avatar) = member.avatar {
            match member_avatar.is_animated() {
                true => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.gif"),
                false => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.png"),
            }
        };

        self.get_user_avatar(user)
    }
}
