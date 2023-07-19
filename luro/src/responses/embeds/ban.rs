use anyhow::Error;
use twilight_model::{
    guild::{Guild, Member},
    id::{marker::GuildMarker, Id},
    user::User,
};
use twilight_util::builder::embed::{
    EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder, ImageSource,
};

use crate::{
    functions::{get_member_avatar, get_user_avatar},
    interactions::InteractionResponse,
    ACCENT_COLOUR,
};

/// Embed showing that a member got banned
pub fn embed(
    guild: Guild,
    moderator: Member,
    banned_user: User,
    reason: &String,
    period: &String,
) -> Result<EmbedBuilder, Error> {
    // Variables for the moderator
    let moderator_avatar = get_member_avatar(Some(&moderator), &Some(guild.id), &moderator.user);
    let moderator_name = if moderator.user.discriminator == 0 {
        moderator.user.name
    } else {
        format!("{}#{}", moderator.user.name, moderator.user.discriminator)
    };

    // Variables for the user that was banned
    let banned_user_avatar = get_user_avatar(&banned_user);
    let banned_user_id = banned_user.id.to_string();
    let banned_user_name = if banned_user.discriminator == 0 {
        banned_user.name
    } else {
        format!("{}#{}", banned_user.name, banned_user.discriminator)
    };

    let embed_author = EmbedAuthorBuilder::new(format!(
        "Banned by {} - {}",
        moderator_name, moderator.user.id
    ))
    .icon_url(ImageSource::url(moderator_avatar)?)
    .build();

    let mut embed = EmbedBuilder::new()
        .color(ACCENT_COLOUR)
        .title(format!("Banned from {}", guild.name))
        .author(embed_author)
        .field(EmbedFieldBuilder::new("Purged Messages", period).inline())
        .field(EmbedFieldBuilder::new("Guild ID", guild.id.to_string()).inline())
        .thumbnail(ImageSource::url(banned_user_avatar)?);

    if !reason.is_empty() {
        embed = embed.description(format!(
            "**User:** <@{banned_user_id}> - {banned_user_name}\n**User ID:** {banned_user_id}\n**Reason:** ```{reason}```",
        ))
    } else {
        embed = embed.description(format!(
            "**User:** <@{banned_user_id}> - {banned_user_name}\n**User ID:** {banned_user_id}",
        ))
    }

    Ok(embed)
}

pub fn interaction_response(
    guild: Guild,
    moderator: Member,
    banned_user: User,
    guild_id: Id<GuildMarker>,
    reason: &String,
    period: &String,
    success: bool,
) -> Result<InteractionResponse, Error> {
    let mut embed = embed(guild, moderator, banned_user, reason, period)?;
    if success {
        embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Successful").inline())
    } else {
        embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Failed").inline())
    }

    Ok(InteractionResponse::Embed {
        embeds: vec![embed.build()],
        components: None,
        ephemeral: false,
    })
}
