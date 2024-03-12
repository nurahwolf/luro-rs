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
}

impl From<twilight_model::guild::Guild> for Guild {
    fn from(twilight_guild: twilight_model::guild::Guild) -> Self {
        Self {
            accent_colour_custom: Default::default(),
            accent_colour: Default::default(),
            moderator_actions_log_channel: Default::default(),
            role_blacklist: Default::default(),
            twilight_guild,
        }
    }
}
