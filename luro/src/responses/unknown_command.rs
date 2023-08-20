use crate::COLOUR_DANGER;

use crate::interaction::LuroSlash;

impl LuroSlash {
    /// A response sent when Luro receives a command it does not have a handler for
    pub async fn unknown_command_response(self) -> anyhow::Result<()> {
        self.respond(|r| {
            r.embed(|embed| {
                embed
                    .title("Unknown Command Received")
                    .colour(COLOUR_DANGER)
                    .description("This command does not exist yet, sorry!")
                    .footer(|footer| footer.text("We had a fucky wucky!"))
            })
            .ephemeral()
        })
        .await
    }

    /// A response sent when Luro receives a command it does not have a handler for
    ///
    /// This version is if we know the name of the failed comman
    pub async fn unknown_command_response_named(self, name: &str) -> anyhow::Result<()> {
        self.respond(|r| {
            r.embed(|embed| {
                embed
                    .title("Unknown Command Received")
                    .colour(COLOUR_DANGER)
                    .description(format!(
                        "The command `{name}` does not yet exist! Really sorry about this! Blame my owner..."
                    ))
                    .footer(|footer| footer.text("We had a fucky wucky!"))
            })
            .ephemeral()
        })
        .await
    }
}
