use anyhow::Error;
use twilight_model::{
    guild::{Guild, Member},
    id::{marker::GuildMarker, Id},
    user::User
};
use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder, ImageSource};

use crate::{
    functions::{get_member_avatar, get_user_avatar},
    interactions::InteractionResponse,
    models::LuroResponse,
    ACCENT_COLOUR
};

/// Embed showing that a member got banned
pub fn kick_embed(
    guild: Guild,
    moderator: Member,
    kicked_user: User,
    guild_id: Id<GuildMarker>,
    reason: &String
) -> Result<EmbedBuilder, Error> {
    // Variables for the moderator
    let moderator_avatar = get_member_avatar(Some(&moderator), &Some(guild_id), &moderator.user);
    let moderator_name = if moderator.user.discriminator == 0 {
        moderator.user.name
    } else {
        format!("{}#{}", moderator.user.name, moderator.user.discriminator)
    };

    // Variables for the user that was banned
    let kicked_user_avatar = get_user_avatar(&kicked_user);
    let kicked_user_id = kicked_user.id.to_string();
    let kicked_user_name = if kicked_user.discriminator == 0 {
        kicked_user.name
    } else {
        format!("{}#{}", kicked_user.name, kicked_user.discriminator)
    };

    let embed_author = EmbedAuthorBuilder::new(format!("Kicked by {} - {}", moderator_name, moderator.user.id))
        .icon_url(ImageSource::url(moderator_avatar)?)
        .build();

    let mut embed = EmbedBuilder::new()
        .color(ACCENT_COLOUR)
        .title(format!("Kicked from {}", guild.name))
        .author(embed_author)
        .field(EmbedFieldBuilder::new("Guild ID", guild.id.to_string()).inline())
        .thumbnail(ImageSource::url(kicked_user_avatar)?);

    if !reason.is_empty() {
        embed = embed.description(format!(
            "**User:** <@{kicked_user_id}> - {kicked_user_name}\n**User ID:** {kicked_user_id}\n**Reason:** ```{reason}```"
        ))
    } else {
        embed = embed.description(format!(
            "**User:** <@{kicked_user_id}> - {kicked_user_name}\n**User ID:** {kicked_user_id}",
        ))
    }

    Ok(embed)
}

pub fn kick_response(
    guild: Guild,
    moderator: Member,
    kicked_user: User,
    guild_id: Id<GuildMarker>,
    reason: &String,
    success: bool,
    luro_response: LuroResponse
) -> Result<InteractionResponse, Error> {
    let mut embed = kick_embed(guild, moderator, kicked_user, guild_id, reason)?;
    if success {
        embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Successful").inline())
    } else {
        embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Failed").inline())
    }

    Ok(InteractionResponse::Embed {
        embeds: vec![embed.build()],
        luro_response
    })
}
