use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
