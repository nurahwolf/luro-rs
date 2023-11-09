use sqlx::types::Json;
use twilight_model::application::interaction::Interaction;

impl crate::SQLxDriver {
    pub async fn update_interaction(&self, interaction: &Interaction) -> Result<u64, sqlx::Error> {
        sqlx::query_file!(
            "queries/interaction_update.sql",
            interaction.app_permissions.map(|x|x.bits() as i64),
            interaction.application_id.get() as i64,
            interaction.channel.as_ref().map(|x|x.id.get() as i64),
            interaction.data.as_ref().map(Json) as _,
            interaction.guild_id.map(|x|x.get() as i64),
            interaction.guild_locale,
            interaction.id.get() as i64,
            Json(interaction.kind) as _,
            interaction.locale,
            interaction.message.as_ref().map(|x|x.id.get() as i64),
            interaction.token,
            interaction.user.as_ref().map(|x|x.id.get() as i64),
        ).execute(&self.pool).await.map(|x|x.rows_affected())
    }
}
