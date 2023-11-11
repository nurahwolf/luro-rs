use luro_model::types::MarriageApprovalsCount;

impl crate::SQLxDriver {
    pub async fn count_marriage_approvers(&self, proposer_id: i64, proposee_id: i64) -> Result<MarriageApprovalsCount, sqlx::Error> {
        sqlx::query_as!(
            MarriageApprovalsCount,
            "
            SELECT 
                COUNT(approve) filter (where approve) as approvers,
                COUNT(disapprove) filter (where disapprove) as disapprovers
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
