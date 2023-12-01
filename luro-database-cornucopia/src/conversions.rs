
impl From<crate::cornucopia::types::public::UserPermissions> for luro_model::types::UserPermissions {
    fn from(permissions: crate::cornucopia::types::public::UserPermissions) -> Self {
        match permissions {
            crate::cornucopia::types::public::UserPermissions::USER => luro_model::types::UserPermissions::User,
            crate::cornucopia::types::public::UserPermissions::OWNER => luro_model::types::UserPermissions::Owner,
            crate::cornucopia::types::public::UserPermissions::ADMINISTRATOR => luro_model::types::UserPermissions::Administrator,
        }
    }
}

impl From<crate::cornucopia::types::public::Sexuality> for luro_model::types::Sexuality {
    fn from(sexuality: crate::cornucopia::types::public::Sexuality) -> Self {
        match sexuality {
            crate::cornucopia::types::public::Sexuality::STRAIGHT => luro_model::types::Sexuality::Straight,
            crate::cornucopia::types::public::Sexuality::BISEXUAL => luro_model::types::Sexuality::Bisexual,
            crate::cornucopia::types::public::Sexuality::PANSEXUAL => luro_model::types::Sexuality::Pansexual,
            crate::cornucopia::types::public::Sexuality::LESBIAN => luro_model::types::Sexuality::Lesbian,
            crate::cornucopia::types::public::Sexuality::GAY => luro_model::types::Sexuality::Gay,
        }
    }
}

impl From<crate::cornucopia::types::public::Gender> for luro_model::types::Gender {
    fn from(gender: crate::cornucopia::types::public::Gender) -> Self {
        match gender {
            crate::cornucopia::types::public::Gender::MALE => luro_model::types::Gender::Male,
            crate::cornucopia::types::public::Gender::FEMALE => luro_model::types::Gender::Female,
            crate::cornucopia::types::public::Gender::TRANS_FEMALE => luro_model::types::Gender::TransFemale,
            crate::cornucopia::types::public::Gender::TRANS_MALE => luro_model::types::Gender::TransMale,
            crate::cornucopia::types::public::Gender::ITS_COMPLICATED => luro_model::types::Gender::ItsComplicated,
        }
    }
}