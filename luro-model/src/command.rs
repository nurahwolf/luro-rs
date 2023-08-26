use std::{future::Future, pin::Pin};

use twilight_model::{
    application::interaction::{
        application_command::CommandData, message_component::MessageComponentInteractionData, modal::ModalInteractionData,
        Interaction
    },
    channel::{Channel, Message},
    guild::{PartialMember, Permissions},
    id::{
        marker::{ApplicationMarker, GuildMarker, InteractionMarker},
        Id
    },
    user::User
};

pub mod prefix_command_group;
pub mod prefix_command_group_emote;
pub mod text_args;

#[derive(Copy, Clone)]
pub struct CommandFlags {
    pub authority: bool,
    pub ephemeral: bool,
    pub only_guilds: bool,
    pub only_owner: bool,
    pub skip_defer: bool
}

// ENUMS
// =====
/// The type of command
// #[derive(Copy, Clone)]
// pub enum CommandKind<D: LuroDatabaseDriver + 'static> {
//     Chat(&'static SlashCommand<D>),
//     Message(&'static MessageCommand<D>)
// }

// TYPES
// =====
pub type PrefixCommandResult<'fut> = Pin<Box<dyn Future<Output = anyhow::Result<()>> + 'fut + Send>>;
pub type InteractionCommandResult = Pin<Box<dyn Future<Output = anyhow::Result<()>> + 'static + Send>>;

// STRUCTURES
// ==========
// pub struct SlashCommand<D: LuroDatabaseDriver> {
//     pub create: fn() -> ApplicationCommandData,
//     // pub exec: fn(LuroContext<D>, InteractionCommand) -> InteractionCommandResult,
//     pub flags: CommandFlags
// }

// pub struct MessageCommand<D: LuroDatabaseDriver> {
//     pub create: fn() -> Command,
//     // pub exec: fn(LuroContext<D>, InteractionCommand) -> InteractionCommandResult,
//     pub flags: CommandFlags,
//     pub name: &'static str
// }

// pub struct PrefixCommand<D: LuroDatabaseDriver> {
//     pub names: &'static [&'static str],
//     pub desc: &'static str,
//     pub help: Option<&'static str>,
//     pub usage: Option<&'static str>,
//     pub examples: &'static [&'static str],
//     pub flags: CommandFlags,
//     pub group: PrefixCommandGroup,
//     // pub exec: for<'f> fn(LuroContext<D>, &'f Message, TextArgs<'f>, Option<Permissions>) -> PrefixCommandResult<'f>
// }

/// A structure containing a list of interaction command
// pub struct InteractionCommands<D: LuroDatabaseDriver + 'static>(Trie<&'static str, CommandKind<D>>);

pub struct InteractionCommand {
    pub application_id: Id<ApplicationMarker>,
    pub channel: Option<Channel>,
    pub data: Box<CommandData>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub id: Id<InteractionMarker>,
    pub latency: twilight_gateway::Latency,
    pub member: Option<PartialMember>,
    pub permissions: Option<Permissions>,
    pub shard: twilight_gateway::MessageSender,
    pub token: String,
    pub user: Option<User>
}

pub struct InteractionComponent {
    pub interaction: Interaction,
    pub application_id: Id<ApplicationMarker>,
    pub channel: Option<Channel>,
    pub data: Box<MessageComponentInteractionData>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub id: Id<InteractionMarker>,
    pub latency: twilight_gateway::Latency,
    pub member: Option<PartialMember>,
    pub message: Message,
    pub permissions: Option<Permissions>,
    pub shard: twilight_gateway::MessageSender,
    pub token: String,
    pub user: Option<User>
}

pub struct InteractionModal {
    pub application_id: Id<ApplicationMarker>,
    pub channel: Option<Channel>,
    pub data: ModalInteractionData,
    pub guild_id: Option<Id<GuildMarker>>,
    pub id: Id<InteractionMarker>,
    pub latency: twilight_gateway::Latency,
    pub member: Option<PartialMember>,
    pub message: Option<Message>,
    pub permissions: Option<Permissions>,
    pub shard: twilight_gateway::MessageSender,
    pub token: String,
    pub user: Option<User>
}
