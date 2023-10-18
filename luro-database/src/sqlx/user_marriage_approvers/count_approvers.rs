use crate::DbUserMarriageApprovalsCount;

impl crate::LuroDatabase {
    pub async fn count_marriage_approvers(&self, proposer_id: i64, proposee_id: i64) -> Result<DbUserMarriageApprovalsCount, sqlx::Error> {
        sqlx::query_as!(
            DbUserMarriageApprovalsCount,
            "
            SELECT 
                COUNT(approve) as approvers,
                COUNT(disapprove) as disapprovers
            FROM 
                user_marriage_approvals
            WHERE
                proposer_id = $1 and proposee_id = $2
            ",
            proposer_id,
            proposee_id
        )
        .fetch_one(&self.pool)
        .await
    }
}
