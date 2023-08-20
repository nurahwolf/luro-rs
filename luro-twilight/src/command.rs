use std::{future::Future, pin::Pin, sync::Arc};

use radix_trie::Trie;
use twilight_interactions::command::ApplicationCommandData;
use twilight_model::{application::command::Command, channel::Message, guild::Permissions};

use crate::{context::Context, interaction::InteractionCommand};

use self::{prefix_command_group::PrefixCommandGroup, text_args::TextArgs};

pub mod command_kind;
pub mod prefix_command;
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
#[derive(Copy, Clone)]
pub enum CommandKind {
    Chat(&'static SlashCommand),
    Message(&'static MessageCommand)
}

// TYPES
// =====
pub type PrefixCommandResult<'fut> = Pin<Box<dyn Future<Output = anyhow::Result<()>> + 'fut + Send>>;
pub type InteractionCommandResult = Pin<Box<dyn Future<Output = anyhow::Result<()>> + 'static + Send>>;

// STRUCTURES
// ==========
pub struct SlashCommand {
    pub create: fn() -> ApplicationCommandData,
    pub exec: fn(Arc<Context>, InteractionCommand) -> InteractionCommandResult,
    pub flags: CommandFlags
}

pub struct MessageCommand {
    pub create: fn() -> Command,
    pub exec: fn(Arc<Context>, InteractionCommand) -> InteractionCommandResult,
    pub flags: CommandFlags,
    pub name: &'static str
}

pub struct PrefixCommand {
    pub names: &'static [&'static str],
    pub desc: &'static str,
    pub help: Option<&'static str>,
    pub usage: Option<&'static str>,
    pub examples: &'static [&'static str],
    pub flags: CommandFlags,
    pub group: PrefixCommandGroup,
    pub exec: for<'f> fn(Arc<Context>, &'f Message, TextArgs<'f>, Option<Permissions>) -> PrefixCommandResult<'f>
}

/// A structure containing a list of interaction command
pub struct InteractionCommands(Trie<&'static str, CommandKind>);
