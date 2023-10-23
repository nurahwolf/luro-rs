use crate::LuroDatabase;

impl LuroDatabase {
    // pub async fn get_users(&self) -> Result<Vec<DatabaseUser>, sqlx::Error> {
    //     sqlx::query_as!(
    //         DatabaseUser,
    //         "SELECT
    //             accent_colour,
    //             user_avatar,
    //             avatar_decoration,
    //             banner,
    //             bot,
    //             characters,
    //             discriminator,
    //             email,
    //             user_flags,
    //             global_name,
    //             locale,
    //             message_edits,
    //             messages,
    //             mfa_enabled,
    //             name,
    //             premium_type,
    //             public_flags,
    //             system,
    //             user_id,
    //             user_permissions as \"user_permissions: LuroUserPermissions\",
    //             verified,
    //             warnings,
    //             words_average,
    //             words_count
    //         FROM
    //             users",
    //     )
    //     .fetch_all(&self.pool)
    //     .await
    // }
}
