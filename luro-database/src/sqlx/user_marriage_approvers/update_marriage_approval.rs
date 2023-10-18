use crate::{LuroDatabase, DbUserMarriageApprovals};

impl LuroDatabase {
    pub async fn update_marriage_approval(&self, marriage: DbUserMarriageApprovals) -> Result<DbUserMarriageApprovals, sqlx::Error> {
        sqlx::query_as!(
            DbUserMarriageApprovals,
            "INSERT INTO user_marriage_approvals (
                approve,
                disapprove,
                proposee_id,
                proposer_id,
                user_id
            ) VALUES
                ($1, $2, $3, $4, $5)
            ON CONFLICT
                (proposer_id, proposee_id, user_id)
            DO UPDATE SET
                approve = $1,
                disapprove = $2
            RETURNING
                approve,
                disapprove,
                proposee_id,
                proposer_id,
                user_id
                ",
                marriage.approve,
                marriage.disapprove,
                marriage.proposee_id,
                marriage.proposer_id,
                marriage.user_id,
        )
        .fetch_one(&self.pool)
        .await
    }
}
