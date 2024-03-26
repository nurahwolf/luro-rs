use futures_util::TryStreamExt;
use twilight_model::id::{marker::UserMarker, Id};

use crate::user::Marriage;

impl crate::database::sqlx::Database {
    pub async fn user_fetch_marriages(&self, user_id: Id<UserMarker>) -> Result<Vec<Marriage>, sqlx::Error> {
        let mut marriages = vec![];
        let mut query = sqlx::query!(
            "SELECT * FROM user_marriages WHERE proposer_id = $1 or proposee_id = $1",
            user_id.get() as i64
        )
        .fetch(&self.pool);

        while let Ok(Some(marriage)) = query.try_next().await {
            marriages.push(Marriage {
                reason: marriage.reason,
                proposee_id: Id::new(marriage.proposee_id as u64),
                proposer_id: Id::new(marriage.proposer_id as u64),
                divorced: marriage.divorced,
                rejected: marriage.rejected,
            })
        }

        Ok(marriages)
    }
}
