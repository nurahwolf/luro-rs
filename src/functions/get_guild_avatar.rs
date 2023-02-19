use twilight_model::{
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
    util::ImageHash,
};

pub fn get_guild_avatar_url(
    guild_id: &Id<GuildMarker>,
    user_id: &Id<UserMarker>,
    avatar: &ImageHash,
) -> String {
    format!(
        "https://cdn.discordapp.com/guilds/{}/users/{}/avatars/{}.png",
        guild_id, user_id, avatar
    )
}

pub fn assemble_user_avatar(
    user_id: &Id<UserMarker>,
    discriminator: u16,
    avatar: Option<&ImageHash>,
) -> String {
    avatar.map_or_else(
        || {
            format!(
                "https://cdn.discordapp.com/embed/avatars/{}.png",
                discriminator % 5
            )
        },
        |avatar| {
            format!(
                "https://cdn.discordapp.com/avatars/{}/{}.png",
                user_id, avatar
            )
        },
    )
}
