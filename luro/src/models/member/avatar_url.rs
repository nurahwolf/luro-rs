impl super::MemberContext {
    /// Return a string that is a link to the user's avatar
    pub fn avatar_url(&self) -> String {
        let id = self.user.user_id;

        if let Some(member_avatar) = self.avatar {
            let guild_id = self.guild_id;
            return match member_avatar.is_animated() {
                true => format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{id}/avatars/{member_avatar}.gif?size=2048"),
                false => format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{id}/avatars/{member_avatar}.png?size=2048"),
            };
        }

        match self.user.avatar {
            Some(avatar) => match avatar.is_animated() {
                true => format!("https://cdn.discordapp.com/avatars/{id}/{avatar}.gif?size=2048"),
                false => format!("https://cdn.discordapp.com/avatars/{id}/{avatar}.png?size=2048"),
            },
            None => format!(
                "https://cdn.discordapp.com/avatars/{}.png?size=2048",
                id.get() > 22 % 6
            ),
        }
    }
}
