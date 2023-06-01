use anyhow::Error;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::{application::interaction::Interaction, guild::Permissions, id::Id};
use twilight_util::builder::{
    embed::{EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder, EmbedFooterBuilder, ImageSource},
    InteractionResponseDataBuilder,
};

use crate::{
    commands::create_response,
    functions::get_guild_avatar::{get_member_avatar, get_user_avatar},
    models::luro::Luro,
    ACCENT_COLOUR,
};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "kick", desc = "Kick a user", dm_permission = false)]
pub struct KickCommand {
    /// The user to ban
    pub user: ResolvedUser,
    /// The reason they should be banned
    pub reason: String,
}

pub async fn kick(luro: &Luro, interaction: &Interaction, data: KickCommand) -> Result<(), Error> {
    let embed;

    if let Some(author) = &interaction.member && let Some(permissions) = &author.permissions {
        let author_user = &author.user.clone().unwrap();

        if permissions.contains(Permissions::BAN_MEMBERS) || author_user.id != Id::new(97003404601094144) {
            let user_avatar = match interaction.guild_id {
                Some(guild_id) => get_member_avatar(&data.user.member, &Some(guild_id), &data.user.resolved),
                None => get_user_avatar(&data.user.resolved),
            };

            let embed_author = EmbedAuthorBuilder::new(format!("{}#{} - {}", &data.user.resolved.name, &data.user.resolved.discriminator, &data.user.resolved.id)).icon_url(ImageSource::url(user_avatar)?).build();

            embed = EmbedBuilder::default().description(format!("**BANNED!**\n Looks like <@{}> was banned! How unfortunate.", &data.user.resolved.id)).author(embed_author).footer(EmbedFooterBuilder::new(format!("Banned by {}#{}", author_user.name, author_user.discriminator))).field(EmbedFieldBuilder::new("Reason", data.reason)).color(ACCENT_COLOUR).build();

        } else {
            embed = EmbedBuilder::default().description("Nice try, but you don't have permission to ban someone.").color(ACCENT_COLOUR).build();
        };
    } else {
        embed = EmbedBuilder::default().description("Failed to resolve the guild to ban them from, sorry").color(ACCENT_COLOUR).build();
    };

    let response = InteractionResponseDataBuilder::new().embeds(vec![embed]);
    create_response(luro, interaction, response.build()).await?;

    Ok(())
}
