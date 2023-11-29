use twilight_model::id::{marker::{MessageMarker, ChannelMarker}, Id};

impl crate::SQLxDriver {
    /// Add a quote to the database, returning the added quote ID
    pub async fn quote_fetch(&self, quote_id: i64) -> anyhow::Result<Option<(Id<MessageMarker>, Id<ChannelMarker>)>> {
        Ok(sqlx::query!(
            "
                SELECT *
                FROM quotes
                WHERE id = $1
            ",
            quote_id
        ).fetch_optional(&self.pool).await?.map(|x|(Id::new(x.message_id as u64), Id::new(x.channel_id as u64))))
    }
}