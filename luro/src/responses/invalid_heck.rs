use luro_builder::embed::EmbedBuilder;
use tracing::warn;

use crate::COLOUR_DANGER;

use crate::interaction::LuroSlash;

impl LuroSlash {
    pub async fn invalid_heck_response(
        self,
        missing_user: bool,
        missing_author: bool,
        heck_message: &str
    ) -> anyhow::Result<()> {
        let mut embed = invalid_heck_embed(heck_message);

        if missing_user {
            embed.field(|field| field.field("Missing Value", "`<user>`", true));
        };
        if missing_author {
            embed.field(|field| field.field("Missing Value", "`<author>`", true));
        };
        self.respond(|response| response.add_embed(embed)).await
    }
}

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
fn invalid_heck_embed(heck_message: &str) -> EmbedBuilder {
    warn!("User attempted to make an invalid heck");
    EmbedBuilder::default()
        .colour(COLOUR_DANGER)
        .title("Can you read?")
        .description(format!("You need to include both `<user>` and `<author>` in your message at least once! For example:\n```<author> exploded <user>!```\nYour heck was:\n```{}```", heck_message)).clone()
}
