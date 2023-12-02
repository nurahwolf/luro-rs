use luro_model::sync::UserSync;

use crate::cornucopia::queries::user_update_twilight_user::user_update_twilight_user;

impl crate::Driver {
    pub async fn user_update(&self, user: impl Into<UserSync<'_>>) -> Result<u64, tokio_postgres::Error> {
        match user.into() {
            UserSync::User(user) => {
                user_update_twilight_user()
                    .bind(
                        &self.client,
                        &user.accent_colour.map(|x| x as i32),
                        &user.avatar_decoration.map(|x| x.to_string()),
                        &user.bot,
                        &(user.discriminator as i16),
                        &user.email,
                        &user.global_name,
                        &user.locale,
                        &user.mfa_enabled,
                        &user.premium_type.map(|x| u8::from(x) as i16),
                        &user.avatar.map(|x| x.to_string()),
                        &user.banner.map(|x| x.to_string()),
                        &(user.user_id.get() as i64),
                        &user.name,
                        &user.system,
                        &user.verified,
                    )
                    .await
            }
            UserSync::CurrentUser(_user) => todo!(),
            UserSync::TwilightUser(_user) => todo!(),
            UserSync::UserID(_user) => todo!(),
            UserSync::UserUpdate(_user) => todo!(),
        }
    }
}
