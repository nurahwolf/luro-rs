use std::{
    fmt::{Display, Formatter},
    str::FromStr
};

use twilight_model::id::{marker::EmojiMarker, Id};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[repr(u8)]
pub enum Emote {
    Std,
    Tko,
    Ctb,
    Mna,

    Osu,
    Twitch,
    Tracking,

    JumpStart,
    SingleStepBack,
    MyPosition,
    SingleStep,
    JumpEnd,

    Miss
}

// impl From<GameMode> for Emote {
//     fn from(mode: GameMode) -> Self {
//         match mode {
//             GameMode::Osu => Self::Std,
//             GameMode::Taiko => Self::Tko,
//             GameMode::Catch => Self::Ctb,
//             GameMode::Mania => Self::Mna,
//         }
//     }
// }

impl FromStr for Emote {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let emote = match s {
            "osu" => Self::Osu,
            "osu_std" => Self::Std,
            "osu_taiko" => Self::Tko,
            "osu_ctb" => Self::Ctb,
            "osu_mania" => Self::Mna,
            "twitch" => Self::Twitch,
            "tracking" => Self::Tracking,
            "jump_start" => Self::JumpStart,
            "single_step_back" => Self::SingleStepBack,
            "my_position" => Self::MyPosition,
            "single_step" => Self::SingleStep,
            "jump_end" => Self::JumpEnd,
            "miss" => Self::Miss,
            _ => return Err(())
        };

        Ok(emote)
    }
}

impl Display for Emote {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // let CustomEmote { id, name } = BotConfig::get().emote(*self);
        let (name, id) = ("uwu", 1234);

        write!(f, "<:{name}:{id}>")
    }
}

#[derive(Debug)]
pub struct CustomEmote {
    pub id: Id<EmojiMarker>,
    pub name: Box<str>
}

impl CustomEmote {
    pub fn new(id: u64, name: Box<str>) -> Self {
        Self { id: Id::new(id), name }
    }
}
