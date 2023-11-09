impl crate::SQLxDriver {
    /// Returns total words, total unique words
    pub async fn count_total_words(&self) -> Result<(i64, i64), sqlx::Error> {
        let query = sqlx::query_file!("queries/user_count_total_words.sql").fetch_all(&self.pool).await?;

        let mut total = (0, 0); // Total words, Total Unique Words
        for message in query {
            if let Some(total_words) = message.total_words {
                total.0 += total_words
            }

            if let Some(total_unique_words) = message.total_unique_words {
                total.1 += total_unique_words
            }
        }

        Ok(total)
    }
}
