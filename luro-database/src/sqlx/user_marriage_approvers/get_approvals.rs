use crate::{DbUserMarriageApprovals, LuroDatabase};

impl LuroDatabase {
    pub async fn get_marriage_approvals(&self, user_id: (i64, i64)) -> Result<Vec<DbUserMarriageApprovals>, sqlx::Error> {
        sqlx::query_as!(
            DbUserMarriageApprovals,
            "SELECT * FROM user_marriage_approvals WHERE proposer_id = $1 and proposee_id = $2",
            user_id.0,
            user_id.1
        )
        .fetch_all(&self.pool)
        .await
    }
}
