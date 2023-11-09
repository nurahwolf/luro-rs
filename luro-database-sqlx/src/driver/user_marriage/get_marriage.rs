use twilight_model::id::{marker::UserMarker, Id};

use super::DbUserMarriage;

impl crate::SQLxDriver {
    pub async fn get_marriage(
        &self,
        proposer_id: Id<UserMarker>,
        proposee_id: Id<UserMarker>,
    ) -> Result<Option<DbUserMarriage>, sqlx::Error> {
        sqlx::query_as!(
            DbUserMarriage,
            "
            SELECT * FROM user_marriages WHERE
                (proposer_id = $1 AND proposee_id = $2)
                    OR
                (proposer_id = $2 AND proposee_id = $1)
            ",
            proposer_id.get() as i64,
            proposee_id.get() as i64,
        )
        .fetch_optional(&self.pool)
        .await
    }
}
