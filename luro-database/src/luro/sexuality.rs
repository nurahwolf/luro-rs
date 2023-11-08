use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, ::sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "sexuality", rename_all = "SCREAMING_SNAKE_CASE")]
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
