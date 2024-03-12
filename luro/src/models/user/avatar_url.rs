impl super::User {
    /// Return a string that is a link to the user's avatar
    pub fn avatar_url(&self) -> String {
        match self {
            Self::User(user) => user.avatar_url(),
            Self::Member(member) => member.avatar_url(),
        }
    }
}

impl super::UserContext {
    /// Return a string that is a link to the user's avatar
    pub fn avatar_url(&self) -> String {
        let id = self.user_id;

        match self.avatar {
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
