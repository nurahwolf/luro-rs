use luro_model::types::GuildData;

impl crate::SQLxDriver {
    pub async fn guild_update_data(&self, data: &GuildData) -> anyhow::Result<u64> {
        Ok(sqlx::query_file!(
            "queries/guild/guild_update_data.sql",
            data.accent_colour_custom.map(|x| x as i32),
            data.accent_colour.map(|x| x as i32),
            data.guild_id.get() as i64,
            data.moderator_actions_log_channel.map(|x| x.get() as i64),
        )
        .execute(&self.pool)
        .await
        .map(|x| x.rows_affected())?)
    }
}
