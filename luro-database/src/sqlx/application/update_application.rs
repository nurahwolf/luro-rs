use crate::{DbApplication, LuroDatabase, sync::ApplicationSync};

impl LuroDatabase {
    pub async fn update_application(&self, data: impl Into<ApplicationSync>) -> Result<DbApplication, sqlx::Error> {
        match data.into() {
            ApplicationSync::PartialApplication(app) => {
                sqlx::query_as!(
                    DbApplication,
                    "INSERT INTO applications (
                    application_id
                ) VALUES
                    ($1)
                ON CONFLICT
                    (application_id)
                DO UPDATE SET
                    application_id = $1
                RETURNING
                    application_id",
                    app.id.get() as i64
                )
                .fetch_one(&self.pool)
                .await
            }
        }
    }
}
