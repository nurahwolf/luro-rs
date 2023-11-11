use luro_model::sync::ApplicationSync;

impl crate::SQLxDriver {
    pub async fn update_application(&self, data: impl Into<ApplicationSync<'_>>) -> Result<u64, sqlx::Error> {
        match data.into() {
            ApplicationSync::PartialApplication(app) => sqlx::query!(
                "INSERT INTO applications (
                    application_id
                ) VALUES
                    ($1)
                ON CONFLICT
                    (application_id)
                DO UPDATE SET
                    application_id = $1
                ",
                app.id.get() as i64
            )
            .execute(&self.pool)
            .await
            .map(|x| x.rows_affected()),
        }
    }
}
