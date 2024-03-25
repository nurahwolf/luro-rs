use twilight_model::id::{
    marker::{ChannelMarker, GuildMarker, RoleMarker},
    Id,
};

pub struct Guild {
    pub accent_colour_custom: Option<u32>,
    pub accent_colour: Option<u32>,
    pub moderator_actions_log_channel: Option<Id<ChannelMarker>>,
    pub role_blacklist: Vec<Id<RoleMarker>>,
    pub twilight_guild: twilight_model::guild::Guild,
}

impl Guild {
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

impl From<twilight_model::guild::Guild> for Guild {
    fn from(twilight_guild: twilight_model::guild::Guild) -> Self {
        Self {
            accent_colour_custom: None,
            accent_colour: None,
            moderator_actions_log_channel: None,
            role_blacklist: Default::default(),
            twilight_guild,
        }
    }
}
