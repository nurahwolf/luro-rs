use crate::sqlx::message::DbWordcount;

impl crate::LuroDatabase {
    /// Returns total words, total unique words
    pub async fn count_total_words(&self) -> Result<(i64, i64), sqlx::Error> {
        let query = sqlx::query_as!(
            DbWordcount,
            r#"
            WITH arranged AS
            (
              SELECT message_id, 
              UNNEST
              (
                STRING_TO_ARRAY
                (
                  REGEXP_REPLACE(content,  '[^\w\s]', '', 'g'), ' '
                )
              ) AS word, 
              content 
              FROM messages
            )  
            SELECT
                a.message_id,
                COUNT(a.word) as total_words,
                COUNT(DISTINCT(a.word)) as total_unique_words,
                a.content as message_content
            FROM arranged a
            GROUP BY a.message_id, a.content;
        "#
        )
        .fetch_all(&self.pool)
        .await?;

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
