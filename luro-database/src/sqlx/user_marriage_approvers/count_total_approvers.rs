use crate::DbUserMarriageApprovalsCount;

impl crate::LuroDatabase {
    pub async fn count_total_marriage_approvers(&self) -> Result<DbUserMarriageApprovalsCount, sqlx::Error> {
        sqlx::query_as!(
            DbUserMarriageApprovalsCount,
            "
            SELECT 
                COUNT(approve) as approvers,
                COUNT(disapprove) as disapprovers
            FROM 
                user_marriage_approvals
            "
        )
        .fetch_one(&self.pool)
        .await
    }
}