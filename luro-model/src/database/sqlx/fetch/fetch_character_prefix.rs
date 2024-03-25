impl crate::database::sqlx::Database {
    pub async fn fetch_character_prefix(&self, user_id: i64, name: &str) -> Result<Option<String>, sqlx::Error> {
        sqlx::query!(
            "
            SELECT prefix FROM user_characters WHERE (user_id = $1 and character_name = $2)
            ",
            user_id,
            name
        )
        .fetch_optional(&self.pool)
        .await
        .map(|x| x.map(|x| x.prefix).unwrap_or(None))
    }
}
