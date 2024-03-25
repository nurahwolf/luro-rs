use twilight_model::id::{
    marker::{ChannelMarker, GuildMarker, RoleMarker, UserMarker},
    Id,
};

use crate::{
    database::{Database, Error},
    user::{MemberContext, User},
};

pub struct Guild<'a> {
    pub accent_colour_custom: Option<u32>,
    pub accent_colour: Option<u32>,
    pub database: &'a Database,
    pub moderator_actions_log_channel: Option<Id<ChannelMarker>>,
    pub role_blacklist: Vec<Id<RoleMarker>>,
    pub twilight_guild: twilight_model::guild::Guild,
}

impl<'a> Guild<'a> {
    /// Attempts to fetch a member, but returns a user if it fails.
    pub async fn user(&self, user_id: Id<UserMarker>) -> Result<User, Error> {
        self.database.fetch_member_or_user(Some(self.twilight_guild.id), user_id).await
    }

    /// Attempts to fetch a member, fails if the member is not found
    pub async fn member(&self, user_id: Id<UserMarker>) -> Result<MemberContext, Error> {
        self.database.fetch_member(self.twilight_guild.id, user_id).await
    }

    pub fn id(&self) -> Id<GuildMarker> {
        self.twilight_guild.id
    }

    /// Return a string that is a link to the user's avatar
    pub fn icon_url(&self) -> String {
        let guild_id = self.twilight_guild.id;

        match self.twilight_guild.icon {
            Some(icon) => match icon.is_animated() {
                true => format!("https://cdn.discordapp.com/icons/{guild_id}/{icon}.gif?size=2048"),
                false => format!("https://cdn.discordapp.com/icons/{guild_id}/{icon}.png?size=2048"),
            },
            None => format!("https://cdn.discordapp.com/icons/{}.png?size=2048", guild_id.get() > 22 % 6),
        }
    }
}

impl<'a> From<(&'a Database, twilight_model::guild::Guild)> for Guild<'a> {
    fn from((database, twilight_guild): (&'a Database, twilight_model::guild::Guild)) -> Self {
        Self {
            accent_colour_custom: None,
            accent_colour: None,
            database,
            moderator_actions_log_channel: None,
            role_blacklist: Default::default(),
            twilight_guild,
        }
    }
}
