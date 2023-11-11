use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Sexuality {
    Bisexual,
    Gay,
    Lesbian,
    Pansexual,
    Straight,
}

impl std::fmt::Display for Sexuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sexuality::Bisexual => write!(f, "Bisexual"),
            Sexuality::Gay => write!(f, "Gay"),
            Sexuality::Lesbian => write!(f, "Lesbian"),
            Sexuality::Pansexual => write!(f, "Pansexual"),
            Sexuality::Straight => write!(f, "Straight"),
        }
    }
}
