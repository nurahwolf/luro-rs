#![feature(async_fn_in_trait)]
#![feature(return_position_impl_trait_in_trait)]

pub mod context;
mod framework;
pub mod interactions;
#[cfg(feature = "responses")]
pub mod responses;
pub mod slash_command;
pub mod traits;

type LuroCommandType = std::collections::HashMap<String, OldLuroCommand>;
type LuroMutex<T> = std::sync::Arc<std::sync::Mutex<T>>;

pub use crate::{
    context::Context,
    framework::Framework,
    interactions::{
        command::CommandInteraction, component::ComponentInteraction, interaction_context::InteractionContext, modal::ModalInteraction,
    },
    slash_command::{CommandResult, LuroCommand as OldLuroCommand},
    traits::{
        create_luro_command::CreateLuroCommand, interaction::InteractionTrait, luro::Luro, luro_command::LuroCommand,
        luro_interaction::LuroInteraction,
    },
};
