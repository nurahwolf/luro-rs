use async_trait::async_trait;
use tracing::{debug, trace};
use twilight_gateway::MessageSender;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::{application::interaction::Interaction, id::Id};
use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedBuilder, EmbedFooterBuilder, ImageSource};

use crate::{
    commands::{
        heck::{format_heck, get_heck},
        LuroCommand
    },
    interactions::InteractionResponse,
    LuroContext, SlashResponse, ACCENT_COLOUR
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
    async fn run_command(self, interaction: Interaction, ctx: LuroContext, _shard: MessageSender) -> SlashResponse {
        let luro_response = ctx.defer_interaction(&interaction, false).await?;
        let (interaction_channel, interaction_author, _) = self.interaction_context(&interaction, "heck someone")?;
        // Is the channel the interaction called in NSFW?
        let nsfw = interaction_channel.nsfw.unwrap_or(false);

        debug!("attempting to get a heck");
        let (heck, heck_id) = get_heck(ctx.clone(), self.id, interaction.guild_id, self.global, nsfw).await?;

        debug!("attempting to format the returned heck");
        let formatted_heck = format_heck(&heck, interaction_author, &self.user.resolved).await;

        // Attempt to get the author of the heck
        debug!("attempting to get the author of the heck");
        let heck_author = match ctx.twilight_cache.user(Id::new(heck.author_id)) {
            Some(ok) => ok.clone(),
            None => ctx.twilight_client.user(Id::new(heck.author_id)).await?.model().await?
        };
        let heck_author_avatar = self.get_user_avatar(&heck_author);
        let embed_author = EmbedAuthorBuilder::new(format!("Heck created by {}", heck_author.name))
            .icon_url(ImageSource::url(heck_author_avatar)?)
            .build();

        // Create our response, depending on if the user wants a plaintext heck or not
        debug!("creating our response");
        Ok(if let Some(plaintext) = self.plaintext && plaintext {
            trace!("user wanted plaintext");
            InteractionResponse::Content { content: formatted_heck.heck_message, luro_response}
        } else {
            trace!("user wanted embed");
            let mut embed = EmbedBuilder::default()
            .description(formatted_heck.heck_message)
            .author(embed_author)
            .color(ACCENT_COLOUR);
        if nsfw {
            embed = embed.footer(EmbedFooterBuilder::new(format!(
                "Heck ID {heck_id} - NSFW Heck"
            )))
        } else {
            embed = embed.footer(EmbedFooterBuilder::new(format!(
                "Heck ID {heck_id} - SFW Heck"
            )))
        }

            InteractionResponse::ContentEmbed { content: format!("<@{}>", self.user.resolved.id), embeds: vec![embed.build()], luro_response }

        })
    }
}
