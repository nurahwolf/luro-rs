use super::prefix_command_group_emote::PrefixCommandGroupEmote;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrefixCommandGroup {
    AllModes,
    Osu,
    Taiko,
    Catch,
    Mania,
    Tracking,
    Twitch,
    Games,
    Utility,
    Songs
}

impl PrefixCommandGroup {
    pub fn emote(self) -> PrefixCommandGroupEmote {
        PrefixCommandGroupEmote { group: self }
    }

    pub fn name(self) -> &'static str {
        match self {
            PrefixCommandGroup::AllModes => "all osu! modes",
            PrefixCommandGroup::Osu => "osu!standard",
            PrefixCommandGroup::Taiko => "osu!taiko",
            PrefixCommandGroup::Catch => "osu!catch",
            PrefixCommandGroup::Mania => "osu!mania",
            PrefixCommandGroup::Tracking => "osu!tracking",
            PrefixCommandGroup::Twitch => "twitch",
            PrefixCommandGroup::Games => "games",
            PrefixCommandGroup::Utility => "utility",
            PrefixCommandGroup::Songs => "songs"
        }
    }
}
