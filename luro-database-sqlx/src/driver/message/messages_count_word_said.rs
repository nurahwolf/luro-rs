impl crate::SQLxDriver {
    /// Returns a word and how common it is, in order
    pub async fn messages_count_word_said(&self, word: &str) -> anyhow::Result<Option<i64>> {
        Ok(sqlx::query_file!("queries/messages_count_word_said.sql", word)
            .fetch_one(&self.pool)
            .await
            .map(|x| x.count)?)
    }
}
