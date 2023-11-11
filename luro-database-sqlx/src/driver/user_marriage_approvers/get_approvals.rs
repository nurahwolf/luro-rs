use luro_model::types::MarriageApprovals;
use twilight_model::id::{marker::UserMarker, Id};

impl crate::SQLxDriver {
    pub async fn marriage_fetch_approvals(
        &self,
        proposer_id: Id<UserMarker>,
        proposee_id: Id<UserMarker>,
    ) -> Result<Vec<MarriageApprovals>, sqlx::Error> {
        sqlx::query!(
            "SELECT * FROM user_marriage_approvals WHERE proposer_id = $1 and proposee_id = $2",
            proposer_id.get() as i64,
            proposee_id.get() as i64,
        )
        .fetch_all(&self.pool)
        .await
        .map(|x| {
            x.into_iter()
                .map(|approval| MarriageApprovals {
                    user_id: Id::new(approval.user_id as u64),
                    proposer_id: Id::new(approval.proposer_id as u64),
                    proposee_id: Id::new(approval.proposee_id as u64),
                    approve: approval.approve,
                    disapprove: approval.disapprove,
                })
                .collect()
        })
    }
}
