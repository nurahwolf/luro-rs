use twilight_model::{
    application::interaction::application_command::InteractionMember,
    guild::Member,
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
    user::User,
    util::ImageHash,
};

pub fn assemble_user_avatar(user: &User) -> String {
    let user_id = user.id;
    user.avatar.map_or_else(
        || {
            format!(
                "https://cdn.discordapp.com/embed/avatars/{}.png",
                user.discriminator % 5
            )
        },
        |avatar| format!("https://cdn.discordapp.com/avatars/{user_id}/{avatar}.png"),
    )
}

/// Return a string that is a link to the member's avatar, falling back to user avatar if it does not exist
pub fn get_member_avatar(
    member: &Option<InteractionMember>,
    guild_id: &Option<Id<GuildMarker>>,
    user: &User,
) -> String {
    let user_id = user.id;

    if let Some(member) = member && let Some(guild_id) = guild_id {
        if let Some(member_avatar) = member.avatar {
            match member_avatar.is_animated() {
                true => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.gif"),
                false => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.png"),
            }
        }
    };

    get_user_avatar(user)
}

/// Return a string that is a link to the user's avatar
pub fn get_user_avatar(user: &User) -> String {
    let user_id = user.id;

    if let Some(user_avatar) = user.avatar {
        match user_avatar.is_animated() {
            true => {
                return format!("https://cdn.discordapp.com/avatars/{user_id}/{user_avatar}.gif")
            }
            false => {
                return format!("https://cdn.discordapp.com/avatars/{user_id}/{user_avatar}.png")
            }
        }
    };

    let modulo = user.discriminator % 5;
    format!("https://cdn.discordapp.com/embed/avatars/{modulo}.png")
}
