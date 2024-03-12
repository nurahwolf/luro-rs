use twilight_model::id::{marker::GuildMarker, Id};

use crate::{database::Database, models::{interaction::InteractionError, MemberContext}};

impl super::User {
    /// Gets a member context, either from within (if already a member), else fetch it from the database
    pub async fn member(
        &self,
        db: &Database,
        guild_id: Id<GuildMarker>,
    ) -> Result<MemberContext, InteractionError> {
        if let super::User::Member(member) = self {
            return Ok(member.clone());
        }

        db.fetch_member(guild_id, self.user_id()).await
    }
}
