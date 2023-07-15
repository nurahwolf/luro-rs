//#![doc = include_str!("../README.md")]

mod parse_impl;

pub mod argument;
pub mod builder;
pub mod command;
pub mod context;
pub mod framework;
pub mod group;
pub mod hook;
pub mod iter;
pub mod modal;
pub mod parse;
pub mod range;
pub mod wait;

// Items used to extract generics from functions, not public API.
#[doc(hidden)]
pub mod extract;

pub use luro_macros as macros;

type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

/// Useful exports to get started quickly
pub mod prelude {
    pub use crate::{
        argument::ArgumentLimits,
        builder::{FrameworkBuilder, WrappedClient},
        context::{AutocompleteContext, Focused, SlashContext},
        framework::{DefaultCommandResult, Framework},
        modal::*,
        parse::{Parse, ParseError},
        range::Range,
    };
    pub use async_trait::async_trait;
    pub use luro_macros::*;
}

pub mod twilight_exports {
    pub use twilight_http::{
        client::{Client, InteractionClient},
        request::application::interaction::UpdateResponse,
        response::DeserializeBodyError,
    };
    pub use twilight_model::{
        application::{
            command::{
                Command, CommandOption, CommandOptionChoice, CommandOptionChoiceValue,
                CommandOptionType,
            },
            interaction::{
                application_command::{
                    CommandData, CommandDataOption, CommandInteractionDataResolved,
                    CommandOptionValue,
                },
                message_component::MessageComponentInteractionData,
                modal::ModalInteractionData,
                Interaction, InteractionData, InteractionType,
            },
        },
        channel::{
            message::{
                component::{ActionRow, TextInput, TextInputStyle},
                Component,
            },
            Message,
        },
        gateway::payload::incoming::InteractionCreate,
        guild::Permissions,
        http::interaction::{
            InteractionResponse, InteractionResponseData, InteractionResponseType,
        },
        id::{
            marker::{
                ApplicationMarker, AttachmentMarker, ChannelMarker, GenericMarker, GuildMarker,
                MessageMarker, RoleMarker, UserMarker,
            },
            Id,
        },
    };
}
