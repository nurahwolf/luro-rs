use tracing::debug;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::id::Id;

use crate::{
    commands::heck::{format_heck, get_heck},
    interaction::LuroSlash,
    luro_command::LuroCommand, functions::client_fetch,
};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "someone", desc = "Heck a user", dm_permission = true)]
pub struct HeckSomeoneCommand {
    /// The user to heck
    pub user: ResolvedUser,
    /// Get a global heck, or a heck that is specific to this server
    pub global: bool,
    /// Get a specific heck
    pub id: Option<i64>,
    /// Should the heck be sent as plaintext? (Without an embed)
    pub plaintext: Option<bool>
}

impl LuroCommand for HeckSomeoneCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        // Is the channel the interaction called in NSFW?
        let interaction = &ctx.interaction;
        let nsfw = interaction.channel.as_ref().unwrap().nsfw.unwrap_or(false);

        // Attempt to get a heck
        let (heck, heck_id) = get_heck(&ctx.framework, self.id, ctx.interaction.guild_id, self.global, nsfw).await?;

        debug!("attempting to format the returned heck");
        let formatted_heck = format_heck(&heck, interaction.author().as_ref().unwrap(), &self.user.resolved).await;

        let slash_author = client_fetch(&ctx.framework, ctx.interaction.guild_id, Id::new(heck.author_id.get())).await?;

        // Create our response, depending on if the user wants a plaintext heck or not
        if let Some(plaintext) = self.plaintext && plaintext {
            ctx.respond(|r|r.content(formatted_heck.heck_message)).await
        } else {
            let accent_colour = ctx.accent_colour().await;
            ctx.respond(|r|r.content(format!("<@{}>", self.user.resolved.id)).embed(|e|e.description(formatted_heck.heck_message).colour(accent_colour).author(|author|author.name(format!("Heck created by {}", slash_author.name)).icon_url(slash_author.avatar)).footer(|f|{
                match nsfw {
                    true => f.text(format!("Heck ID {heck_id} - NSFW Heck")),
                    false => f.text(format!("Heck ID {heck_id} - SFW Heck")),
                }
            }))).await
        }
    }
}
