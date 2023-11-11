use luro_model::types::Sexuality;

#[derive(Debug, Clone, ::sqlx::Type)]
#[sqlx(type_name = "sexuality", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DbSexuality {
    Bisexual,
    Gay,
    Lesbian,
    Pansexual,
    Straight,
}

impl From<Sexuality> for DbSexuality {
    fn from(sexuality: Sexuality) -> Self {
        match sexuality {
            Sexuality::Bisexual => Self::Bisexual,
            Sexuality::Gay => Self::Gay,
            Sexuality::Lesbian => Self::Lesbian,
            Sexuality::Pansexual => Self::Pansexual,
            Sexuality::Straight => Self::Straight,
        }
    }
}

impl From<DbSexuality> for Sexuality {
    fn from(sexuality: DbSexuality) -> Self {
        match sexuality {
            DbSexuality::Bisexual => Self::Bisexual,
            DbSexuality::Gay => Self::Gay,
            DbSexuality::Lesbian => Self::Lesbian,
            DbSexuality::Pansexual => Self::Pansexual,
            DbSexuality::Straight => Self::Straight,
        }
    }
}
