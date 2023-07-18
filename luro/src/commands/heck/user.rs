use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::{
    application::interaction::Interaction, http::interaction::InteractionResponseType, id::Id,
};
use twilight_util::builder::{
    embed::{EmbedAuthorBuilder, EmbedBuilder, EmbedFooterBuilder, ImageSource},
    InteractionResponseDataBuilder,
};

use crate::{
    commands::heck::{format_heck, get_heck},
    functions::{get_user_avatar, interaction_context},
    interactions::InteractionResponse,
    LuroContext, SlashResponse, ACCENT_COLOUR,
};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "someone", desc = "Heck a user", dm_permission = true)]
pub struct HeckUserCommand {
    /// The user to heck
    pub user: ResolvedUser,
    /// Get a specific heck
    pub id: Option<i64>,
    /// Should the heck be sent as plaintext? (Without an embed)
    pub plaintext: Option<bool>,
}

impl HeckUserCommand {
    pub async fn run(self, ctx: LuroContext, interaction: &Interaction) -> SlashResponse {
        let (interaction_channel, interaction_author, _) =
            interaction_context(interaction, "heck user")?;
        // Is the channel the interaction called in NSFW?
        let nsfw = interaction_channel.nsfw.unwrap_or(false);

        let (heck, heck_id) = get_heck(ctx.clone(), self.id, nsfw).await?;
        let formatted_heck = format_heck(&heck, interaction_author, &self.user.resolved).await;

        // Attempt to get the author of the heck
        let heck_author = match ctx.twilight_cache.user(Id::new(heck.author_id)) {
            Some(ok) => ok.clone(),
            None => {
                ctx.twilight_client
                    .user(Id::new(heck.author_id))
                    .await?
                    .model()
                    .await?
            }
        };
        let heck_author_avatar = get_user_avatar(&heck_author);
        let embed_author = EmbedAuthorBuilder::new(format!("Heck created by {}", heck_author.name))
            .icon_url(ImageSource::url(heck_author_avatar)?)
            .build();

        // Create our response, depending on if the user wants a plaintext heck or not
        let mut response = InteractionResponseDataBuilder::new();
        if let Some(plaintext) = self.plaintext && plaintext {
            response = response.content(formatted_heck.heck_message)
        } else {
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

         response = response.embeds(vec![embed.build()])
         .content(format!("<@{}>", self.user.resolved.id))
        }

        Ok(InteractionResponse::Raw {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(response.build()),
        })
    }
}
