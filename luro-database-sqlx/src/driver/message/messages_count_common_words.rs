impl crate::SQLxDriver {
    /// Returns a word and how common it is, in order
    pub async fn messages_count_common_words(&self) -> anyhow::Result<Vec<(String, i64)>> {
        Ok(sqlx::query_file!("queries/messages_count_common_words.sql")
            .fetch_all(&self.pool)
            .await
            .map(|x| {
                x.into_iter()
                    .map(|words| (words.word, words.word_count.unwrap_or_default()))
                    .collect()
            })?)
    }
}
