use tracing::warn;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};

use crate::COLOUR_DANGER;

use crate::models::LuroSlash;

impl LuroSlash {
    pub async fn invalid_heck_response(
        mut self,
        missing_user: bool,
        missing_author: bool,
        heck_message: &str
    ) -> anyhow::Result<()> {
        let mut embed = invalid_heck_embed(heck_message);

        if missing_user {
            embed = embed.field(EmbedFieldBuilder::new("Missing Value", "`<user>`").inline())
        };
        if missing_author {
            embed = embed.field(EmbedFieldBuilder::new("Missing Value", "`<author>`").inline())
        };
        self.embed(embed.build())?.respond().await
    }
}

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
fn invalid_heck_embed(heck_message: &str) -> EmbedBuilder {
    warn!("User attempted to make an invalid heck");
    EmbedBuilder::new()
        .color(COLOUR_DANGER)
        .title("Can you read?")
        .description(format!("You need to include both `<user>` and `<author>` in your message at least once! For example:\n```<author> exploded <user>!```\nYour heck was:\n```{}```", heck_message))
}
