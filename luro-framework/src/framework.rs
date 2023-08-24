use luro_model::database::drivers::LuroDatabaseDriver;

use crate::Framework;

#[cfg(feature = "luro-builder")]
mod default_embed;
mod guild_accent_colour;
mod interaction_client;
mod register_global_commands;
mod register_guild_commands;
#[cfg(feature = "luro-builder")]
mod send_log_channel;
#[cfg(feature = "luro-builder")]
mod send_message;

impl<D: LuroDatabaseDriver> Framework<D> {

}