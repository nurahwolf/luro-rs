use luro_model::database::drivers::LuroDatabaseDriver;
use tracing::error;
use twilight_model::id::{marker::GuildMarker, Id};
use twilight_util::builder::embed::{EmbedBuilder, EmbedFooterBuilder};

use crate::COLOUR_DANGER;

use crate::interaction::LuroSlash;

impl<D: LuroDatabaseDriver,> LuroSlash<D,> {
    pub async fn no_guild_settings_response(&self, guild_id: Id<GuildMarker,>,) -> anyhow::Result<(),> {
        self.respond(|r| r.add_embed(no_guild_settings_embed(guild_id,).build(),),)
            .await
    }
}

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
fn no_guild_settings_embed(guild_id: Id<GuildMarker,>,) -> EmbedBuilder {
    error!("No guild settings present for guild {}", guild_id);
    EmbedBuilder::new()
        .title("No Guild Settings")
        .color(COLOUR_DANGER)
        .description(format!("Looks like this guild (<#{}>) has not been enrolled into my guild settings. That's not ideal. Try running the command again?", guild_id))
        .footer(EmbedFooterBuilder::new(
            "Okay, Houston, I believe we've had a problem here ...",
        ))
}
