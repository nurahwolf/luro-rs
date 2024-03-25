// This module defines a set of standard responses, that can be used with the framework, or standalone.
//
// Each response should contain the following two items:
//
// - A public function that takes the bare minimum of data required to create the response, without the framework
// - An implementation on [Response], which uses the framework for creating a response.

pub mod not_guild;
mod punishment;

pub use punishment::{Punishment, PunishmentData};
use twilight_model::id::{marker::GuildMarker, Id};

use crate::context::SlashContext;

/// A wrapper around each individual response, to add useful functionality such as logging and automatic guild colours.
///
/// This is an enum that wraps the different types of 'Contexts' the responses could be called from.
pub enum Response<'a> {
    /// A response crafted in a slash command context.
    SlashCommand(&'a SlashContext<'a>),
}

impl<'a> Response<'a> {
    /// Returns the guild id, if this interaction is a guild.
    pub fn guild_id(&self) -> Option<Id<GuildMarker>> {
        match self {
            Response::SlashCommand(ctx) => ctx.guild.as_ref().map(|guild| guild.twilight_guild.id),
        }
    }

    /// Returns an accent colour following the below formula:
    ///
    /// User's Accent Colour -> Guild's Accent Colour -> Hardcoded accent colour
    pub fn accent_colour(&self) -> u32 {
        self.user_accent_colour()
            .unwrap_or(self.guild_accent_colour().unwrap_or(crate::COLOUR_DEFAULT))
    }

    /// Returns an accent colour for the user invoking the response.
    pub fn user_accent_colour(&self) -> Option<u32> {
        None
    }

    /// Returns an accent colour for the guild the response is in, if present.
    pub fn guild_accent_colour(&self) -> Option<u32> {
        None
    }
}
