use luro_model::types::Gender;

#[derive(Debug, Clone, ::sqlx::Type)]
#[sqlx(type_name = "gender", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DbGender {
    Female,
    ItsComplicated,
    Male,
    TransFemale,
    TransMale,
}

impl From<Gender> for DbGender {
    fn from(gender: Gender) -> Self {
        match gender {
            Gender::Female => Self::Female,
            Gender::ItsComplicated => Self::ItsComplicated,
            Gender::Male => Self::Male,
            Gender::TransFemale => Self::TransFemale,
            Gender::TransMale => Self::TransMale,
        }
    }
}

impl From<DbGender> for Gender {
    fn from(gender: DbGender) -> Self {
        match gender {
            DbGender::Female => Self::Female,
            DbGender::ItsComplicated => Self::ItsComplicated,
            DbGender::Male => Self::Male,
            DbGender::TransFemale => Self::TransFemale,
            DbGender::TransMale => Self::TransMale,
        }
    }
}