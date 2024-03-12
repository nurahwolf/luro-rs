#[cfg(not(feature = "database-sqlx"))]
#[derive(Clone, Debug, serde::Deserialize, PartialEq, serde::Serialize)]
pub enum Gender {
    Female,
    ItsComplicated,
    Male,
    TransFemale,
    TransMale,
}

#[cfg(feature = "database-sqlx")]
#[derive(Clone, Debug, serde::Deserialize, PartialEq, serde::Serialize, ::sqlx::Type)]
#[sqlx(type_name = "gender", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Gender {
    Female,
    ItsComplicated,
    Male,
    TransFemale,
    TransMale,
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::Female => write!(f, "Female"),
            Gender::ItsComplicated => write!(f, "It's Complicated"),
            Gender::Male => write!(f, "Male"),
            Gender::TransFemale => write!(f, "Female (Trans)"),
            Gender::TransMale => write!(f, "Male (Trans)"),
        }
    }
}

#[cfg(not(feature = "database-sqlx"))]
#[derive(Clone, Debug, serde::Deserialize, PartialEq, serde::Serialize)]
pub enum Sexuality {
    Bisexual,
    Gay,
    Lesbian,
    Pansexual,
    Straight,
}

#[cfg(feature = "database-sqlx")]
#[derive(Clone, Debug, serde::Deserialize, PartialEq, serde::Serialize, ::sqlx::Type)]
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
