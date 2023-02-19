use twilight_model::{
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
    util::ImageHash,
};

pub fn get_user_avatar_url(user_id: &Id<UserMarker>, avatar: &ImageHash) -> String {
    format!("https://cdn.discordapp.com/avatars/{user_id}/{avatar}.png")
}

pub fn get_guild_avatar_url(
    guild_id: &Id<GuildMarker>,
    user_id: &Id<UserMarker>,
    avatar: &ImageHash,
) -> String {
    format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{avatar}.png")
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
        |avatar| format!("https://cdn.discordapp.com/avatars/{user_id}/{avatar}.png"),
    )
}
