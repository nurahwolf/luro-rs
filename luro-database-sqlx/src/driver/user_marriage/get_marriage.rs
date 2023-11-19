use twilight_model::id::{marker::UserMarker, Id};

use super::DbUserMarriage;

impl crate::SQLxDriver {
    pub async fn get_marriage(
        &self,
        proposer_id: Id<UserMarker>,
        proposee_id: Id<UserMarker>,
    ) -> Result<Option<DbUserMarriage>, sqlx::Error> {
        let proposee = proposee_id.min(proposer_id).get() as i64;
        let proposer = proposee_id.max(proposer_id).get() as i64;
        sqlx::query_as!(
            DbUserMarriage,
            "
            SELECT * FROM user_marriages WHERE
                (proposer_id = $1 AND proposee_id = $2)
                    OR
                (proposer_id = $2 AND proposee_id = $1)
            ",
            proposer,
            proposee,
        )
        .fetch_optional(&self.pool)
        .await
    }
}
