use crate::{DbUserMarriage, LuroDatabase};

impl LuroDatabase {
    pub async fn update_marriage(&self, marriage: DbUserMarriage) -> Result<DbUserMarriage, sqlx::Error> {
        sqlx::query_as!(
            DbUserMarriage,
            "INSERT INTO user_marriages (
                active,
                proposee_id,
                proposer_id,
                reason,
                rejected
            ) VALUES
                ($1, $2, $3, $4, $5)
            ON CONFLICT
                (proposer_id, proposee_id)
            DO UPDATE SET
                active = $1,
                reason = $4,
                rejected = $5
            RETURNING
                active,
                proposee_id,
                proposer_id,
                reason,
                rejected
                ",
                marriage.active,
                marriage.proposee_id,
                marriage.proposer_id,
                marriage.reason,
                marriage.rejected,
        )
        .fetch_one(&self.pool)
        .await
    }
}
