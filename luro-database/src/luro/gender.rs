use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, ::sqlx::Type, Serialize, Deserialize)]
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