use std::fmt::{Display, Formatter};

use crate::emote::Emote;

use super::prefix_command_group::PrefixCommandGroup;

pub struct PrefixCommandGroupEmote {
    pub group: PrefixCommandGroup,
}

impl Display for PrefixCommandGroupEmote {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.group {
            PrefixCommandGroup::AllModes => Display::fmt(&Emote::Osu, f),
            PrefixCommandGroup::Osu => Display::fmt(&Emote::Std, f),
            PrefixCommandGroup::Taiko => Display::fmt(&Emote::Tko, f),
            PrefixCommandGroup::Catch => Display::fmt(&Emote::Ctb, f),
            PrefixCommandGroup::Mania => Display::fmt(&Emote::Mna, f),
            PrefixCommandGroup::Tracking => Display::fmt(&Emote::Tracking, f),
            PrefixCommandGroup::Twitch => Display::fmt(&Emote::Twitch, f),
            PrefixCommandGroup::Games => f.write_str(":video_game:"),
            PrefixCommandGroup::Utility => f.write_str(":tools:"),
            PrefixCommandGroup::Songs => f.write_str(":musical_note:"),
        }
    }
}