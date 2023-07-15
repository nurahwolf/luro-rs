
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::{application::interaction::Interaction, http::interaction::InteractionResponseType, id::Id};
use twilight_util::builder::{
    embed::{EmbedAuthorBuilder, EmbedBuilder, EmbedFooterBuilder, ImageSource},
    InteractionResponseDataBuilder,
};

use crate::{interactions::InteractionResponse, commands::heck::{get_heck, format_heck}, functions::get_user_avatar, ACCENT_COLOUR, responses::embeds::{unable_to_get_guild::unable_to_get_guild, internal_error::internal_error}, LuroContext};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "user", desc = "Heck a user", dm_permission = true)]
pub struct HeckUserCommand {
    /// The user to heck
    pub user: ResolvedUser,
}

impl HeckUserCommand {
    pub async fn run(
        self,
        ctx: LuroContext,
        interaction: &Interaction,
    ) -> anyhow::Result<InteractionResponse> {
        tracing::debug!(
            "heck user command in channel {} by {}",
            interaction.channel.clone().unwrap().name.unwrap(),
            interaction.user.clone().unwrap().name
        );
    
        let guild_id = match interaction.guild_id {
            Some(guild_id) => guild_id,
            None => return Ok(unable_to_get_guild("Failed to get guild ID".to_string())),
        };
        let user_id = match &interaction.user {
            Some(user) => user.id,
            None => match &interaction.member {
                Some(member) => member.clone().user.unwrap().id,
                None => return Ok(internal_error("Failed to find the user who invoked this interaction".to_string())),
            },
        };

        let author = ctx.twilight_client.guild_member(guild_id, user_id).await?.model().await?;
    
        let channel = ctx
            .twilight_client
            .channel(interaction.channel.clone().unwrap().id)
            .await?
            .model()
            .await?;
        let nsfw = channel.nsfw.unwrap_or(false);
    
        let (heck, heck_id) = get_heck(ctx.clone(), None, nsfw).await?;
        let heckee = ctx.twilight_client.user(Id::new(heck.author_id)).await?.model().await?;
        let heckee_avatar: String = get_user_avatar(&heckee);
        let heck = format_heck(&heck, &self.user.resolved, &heckee).await;
    
        let embed_author = EmbedAuthorBuilder::new(format!("Heck created by {}", author.user.name))
            .icon_url(ImageSource::url(heckee_avatar)?)
            .build();
        let mut embed = EmbedBuilder::default()
            .description(heck.heck_message)
            .author(embed_author)
            .color(ACCENT_COLOUR);
        if nsfw {
            embed = embed.footer(EmbedFooterBuilder::new(format!("Heck ID {heck_id} - NSFW Heck")))
        } else {
            embed = embed.footer(EmbedFooterBuilder::new(format!("Heck ID {heck_id} - SFW Heck")))
        }

        embed = embed.footer(EmbedFooterBuilder::new(format!(
            "Heck {} - NSFW: {}",
            heck_id, nsfw
        )));
    
            let response = InteractionResponseDataBuilder::new()
            .embeds(vec![embed.build()])
            .content(format!("<@{}>", self.user.resolved.id)).build();
    
        Ok(InteractionResponse::Raw { kind: InteractionResponseType::ChannelMessageWithSource, data: Some(response) })
    }
}
