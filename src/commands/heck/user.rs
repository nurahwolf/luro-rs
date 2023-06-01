use anyhow::Error;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::{application::interaction::Interaction, id::Id};
use twilight_util::builder::{
    embed::{EmbedAuthorBuilder, EmbedBuilder, EmbedFooterBuilder, ImageSource},
    InteractionResponseDataBuilder,
};

use crate::{
    commands::{
        create_response,
        heck::{format_heck, get_heck},
    },
    functions::get_guild_avatar::{get_member_avatar, get_user_avatar},
    models::luro::Luro,
    ACCENT_COLOUR,
};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "user", desc = "Heck a user", dm_permission = true)]
pub struct HeckUserCommand {
    /// The user to heck
    pub user: ResolvedUser,
}

pub async fn user(
    luro: &Luro,
    interaction: &Interaction,
    data: HeckUserCommand,
) -> Result<(), Error> {
    tracing::debug!(
        "heck user command in channel {} by {}",
        interaction.channel.clone().unwrap().name.unwrap(),
        interaction.user.clone().unwrap().name
    );

    let channel = luro
        .twilight_client
        .channel(interaction.channel.clone().unwrap().id)
        .await?
        .model()
        .await?;
    let nsfw = channel.nsfw.unwrap_or(false);

    let (heck, heck_id) = get_heck(&luro.global_command_data, None, nsfw).await?;
    let author = luro
        .twilight_client
        .user(Id::new(heck.author_id))
        .await?
        .model()
        .await?;
    let heck = format_heck(&heck, &author, &data.user.resolved).await;

    let author_avatar = match interaction.guild_id {
        Some(guild_id) => {
            get_member_avatar(&data.user.member, &Some(guild_id), &data.user.resolved)
        }
        None => get_user_avatar(&data.user.resolved),
    };

    let embed_author = EmbedAuthorBuilder::new(format!("Heck created by {}", author.name))
        .icon_url(ImageSource::url(author_avatar)?)
        .build();
    let embed = EmbedBuilder::default()
        .description(&heck.heck_message)
        .author(embed_author)
        .footer(EmbedFooterBuilder::new(format!(
            "Heck {} - NSFW: {}",
            heck_id, nsfw
        )))
        .color(ACCENT_COLOUR)
        .build();

    let response = InteractionResponseDataBuilder::new()
        .embeds(vec![embed])
        .content(format!("<@{}>", data.user.resolved.id));
    create_response(luro, interaction, response.build()).await?;

    Ok(())
}
