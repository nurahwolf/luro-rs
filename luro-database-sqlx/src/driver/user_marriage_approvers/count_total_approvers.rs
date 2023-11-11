use luro_model::types::MarriageApprovalsCount;

impl crate::SQLxDriver {
    pub async fn count_total_marriage_approvers(&self) -> Result<MarriageApprovalsCount, sqlx::Error> {
        sqlx::query_as!(
            MarriageApprovalsCount,
            "
            SELECT 
                COUNT(approve) filter (where approve) as approvers,
                COUNT(disapprove) filter (where disapprove) as disapprovers
            FROM 
                user_marriage_approvals
            "
        )
        .fetch_one(&self.pool)
        .await
    }
}
