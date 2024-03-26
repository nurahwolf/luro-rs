use twilight_model::id::{marker::UserMarker, Id};

use crate::user::Marriage;

impl crate::database::sqlx::Database {
    pub async fn delete_marriage(
        &self,
        first_user: Id<UserMarker>,
        second_user: Id<UserMarker>,
    ) -> Result<Option<Marriage>, sqlx::Error> {
        let proposee_id = first_user.min(second_user);
        let proposer_id = first_user.max(second_user);
        sqlx::query!(
            "
            DELETE FROM user_marriages
            WHERE (proposer_id, proposee_id) IN (select proposer_id, proposee_id from user_marriages where
                (proposer_id = $1 AND proposee_id = $2)
                    or
                (proposer_id = $2 AND proposee_id = $1)
            )
            RETURNING *
            ",
            proposee_id.get() as i64,
            proposer_id.get() as i64,
        )
        .fetch_optional(&self.pool)
        .await
        .map(|some| {
            some.map(|marriage| Marriage {
                proposer_id,
                proposee_id,
                divorced: marriage.divorced,
                rejected: marriage.rejected,
                reason: marriage.reason,
            })
        })
    }
}
