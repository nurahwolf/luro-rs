use luro_model::sync::UserSync;
use tracing::debug;

impl crate::SQLxDriver {
    pub async fn update_user(&self, user: impl Into<UserSync<'_>>) -> anyhow::Result<u64> {
        let rows_modified = match user.into() {
            UserSync::User(user) => {
                sqlx::query_file!(
                    "queries/users/update_twilight_user.sql",
                    user.accent_colour.map(|x| x as i32),
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
                    user.user_id.get() as i64,
                    user.name,
                    user.system,
                    user.verified
                )
                .execute(&self.pool)
                .await?
            }
            UserSync::CurrentUser(user) => {
                sqlx::query_file!(
                    "queries/user_update_current_user.sql",
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
                .execute(&self.pool)
                .await?
            }
            UserSync::TwilightUser(user) => {
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
                .execute(&self.pool)
                .await?
            }
            UserSync::UserID(user) => {
                sqlx::query_file!("queries/users/update_twilight_user_id.sql", user.get() as i64)
                    .execute(&self.pool)
                    .await?
            }
            UserSync::UserUpdate(user) => {
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
                .execute(&self.pool)
                .await?
            }
        }
        .rows_affected();

        debug!("DB Member: Updated `{rows_modified}` rows!");

        Ok(rows_modified)
    }
}
