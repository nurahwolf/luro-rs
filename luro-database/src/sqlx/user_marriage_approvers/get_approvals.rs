use twilight_model::id::{marker::UserMarker, Id};

use crate::{DbUserMarriageApprovals, LuroDatabase};

impl LuroDatabase {
    pub async fn get_marriage_approvals(
        &self,
        proposer_id: Id<UserMarker>,
        proposee_id: Id<UserMarker>,
    ) -> Result<Vec<DbUserMarriageApprovals>, sqlx::Error> {
        sqlx::query_as!(
            DbUserMarriageApprovals,
            "SELECT * FROM user_marriage_approvals WHERE proposer_id = $1 and proposee_id = $2",
            proposer_id.get() as i64,
            proposee_id.get() as i64,
        )
        .fetch_all(&self.pool)
        .await
    }
}
