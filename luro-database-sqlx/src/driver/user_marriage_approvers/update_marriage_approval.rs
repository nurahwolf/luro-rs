use luro_model::types::MarriageApprovals;
use twilight_model::id::Id;

impl crate::SQLxDriver {
    pub async fn marriage_update_approvals(&self, marriage: MarriageApprovals) -> Result<MarriageApprovals, sqlx::Error> {
        sqlx::query_file!(
            "queries/marriage/marriage_update_approvals.sql",
            marriage.approve,
            marriage.disapprove,
            marriage.proposee_id.get() as i64,
            marriage.proposer_id.get() as i64,
            marriage.user_id.get() as i64,
        )
        .fetch_one(&self.pool)
        .await
        .map(|approvers| MarriageApprovals {
            user_id: Id::new(approvers.user_id as u64),
            proposer_id: Id::new(approvers.proposer_id as u64),
            proposee_id: Id::new(approvers.proposee_id as u64),
            approve: approvers.approve,
            disapprove: approvers.disapprove,
        })
    }
}
