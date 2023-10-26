use sqlx::postgres::PgQueryResult;

use crate::LuroDatabase;

impl LuroDatabase {
    pub async fn delete_role(&self, role_id: i64) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query_as!(
            DbRole,
            "INSERT INTO guild_roles (
                deleted,
                role_id
            ) VALUES
                ($1, $2)
            ON CONFLICT
                (role_id, guild_id)
            DO UPDATE SET
                deleted = $1",
            true,
            role_id,
        )
        .execute(&self.pool)
        .await
    }
}
