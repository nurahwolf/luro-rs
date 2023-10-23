use sqlx::{Error, postgres::PgQueryResult};
use tracing::debug;
use twilight_model::{user::User, gateway::payload::incoming::UserUpdate};

use crate::{DatabaseUserType, LuroDatabase};

impl LuroDatabase {
    pub async fn update_user(&self, user: impl Into<DatabaseUserType>) -> anyhow::Result<u64> {
        let rows_modified = match user.into() {
            DatabaseUserType::User(user) => handle_user(self, user).await?.rows_affected(),
            // DatabaseUserType::LuroUser(user) => self.handle_luro_user(user).await,
            DatabaseUserType::LuroUser(_user) => todo!(),
            DatabaseUserType::UserUpdate(user) => handle_user_update(self, user).await?.rows_affected(),
        };

        debug!("DB Member: Updated `{rows_modified}` rows!");

        Ok(rows_modified)
    }
}

async fn handle_user(db: &LuroDatabase, user: User) -> Result<PgQueryResult, Error> {
    sqlx::query_file!(
        "queries/users/update_twilight_user.sql",
        user.accent_color.map(|x| x as i32),
        user.avatar_decoration.map(|x| x.to_string()),
        user.bot,
        user.discriminator as i16,
        user.email,
        user.global_name,
        user.locale,
        user.mfa_enabled,
        user.premium_type.map(|x| u8::from(x) as i16),
        user.public_flags.map(|x| x.bits() as i64),
        user.avatar.map(|x| x.to_string()),
        user.banner.map(|x| x.to_string()),
        user.flags.map(|x| x.bits() as i64),
        user.id.get() as i64,
        user.name,
        user.system,
        user.verified
    )
    .execute(&db.pool)
    .await
}

async fn handle_user_update(db: &LuroDatabase, user: UserUpdate) -> Result<PgQueryResult, Error> {
    sqlx::query_file!(
        "queries/users/update_twilight_user_update.sql",
        user.accent_color.map(|x| x as i32),
        user.bot,
        user.discriminator as i16,
        user.email,
        user.locale,
        user.mfa_enabled,
        user.premium_type.map(|x| u8::from(x) as i16),
        user.public_flags.map(|x| x.bits() as i64),
        user.avatar.map(|x| x.to_string()),
        user.banner.map(|x| x.to_string()),
        user.flags.map(|x| x.bits() as i64),
        user.id.get() as i64,
        user.name,
        user.verified
    )
    .execute(&db.pool)
    .await
}