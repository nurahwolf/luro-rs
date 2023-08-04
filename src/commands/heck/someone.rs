use async_trait::async_trait;
use tracing::{debug, trace};

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::id::Id;
use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedFooterBuilder, ImageSource};

use crate::{
    commands::heck::{format_heck, get_heck},
    models::LuroSlash,
    traits::{luro_command::LuroCommand, luro_functions::LuroFunctions}
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

#[async_trait]
impl LuroCommand for HeckSomeoneCommand {
    async fn run_command(self, mut ctx: LuroSlash) -> anyhow::Result<()> {
        // Is the channel the interaction called in NSFW?
        let nsfw = ctx.channel()?.nsfw.unwrap_or(false);

        debug!("attempting to get a heck");
        let (heck, heck_id) = get_heck(&ctx.luro, self.id, ctx.interaction.guild_id, self.global, nsfw).await?;

        debug!("attempting to format the returned heck");
        let formatted_heck = format_heck(&heck, &ctx.author()?, &self.user.resolved).await;

        // Attempt to get the author of the heck
        debug!("attempting to get the author of the heck");
        let heck_author = match ctx.luro.twilight_cache.user(Id::new(heck.author_id)) {
            Some(ok) => ok.clone(),
            None => ctx.luro.twilight_client.user(Id::new(heck.author_id)).await?.model().await?
        };
        let heck_author_avatar = ctx.user_get_avatar(&heck_author);
        let embed_author = EmbedAuthorBuilder::new(format!("Heck created by {}", heck_author.name))
            .icon_url(ImageSource::url(heck_author_avatar)?)
            .build();

        // Create our response, depending on if the user wants a plaintext heck or not
        debug!("creating our response");
        if let Some(plaintext) = self.plaintext && plaintext {
            trace!("user wanted plaintext");
            ctx.content(formatted_heck.heck_message).respond().await
        } else {
            trace!("user wanted embed");
            let mut embed = ctx.default_embed().await?
            .description(formatted_heck.heck_message)
            .author(embed_author);
        if nsfw {
            embed = embed.footer(EmbedFooterBuilder::new(format!(
                "Heck ID {heck_id} - NSFW Heck"
            )))
        } else {
            embed = embed.footer(EmbedFooterBuilder::new(format!(
                "Heck ID {heck_id} - SFW Heck"
            )))
        }
            ctx.content(format!("<@{}>", self.user.resolved.id)).embed(embed.build())?.respond().await
        }
    }
}
