use time::OffsetDateTime;
use twilight_model::guild::Member;
use twilight_model::util::ImageHash;
use sqlx::types::Json;

use crate::{LuroDatabase, DbMember};

pub enum DbMemberType {
    Member(Member)
}

impl LuroDatabase {
    pub async fn update_member(&self, member: impl Into<DbMemberType>, guild_id: i64) -> Result<Option<DbMember>, sqlx::Error> {
        let member = member.into();

        match member {
            DbMemberType::Member(member) => self.handle_member(member, guild_id).await,
        }
    }

    async fn handle_member(&self, member: Member, guild_id: i64) -> Result<Option<DbMember>, sqlx::Error> {
        sqlx::query_as!(
            DbMember,
            "INSERT INTO guild_members (
                user_id,
                guild_id,
                avatar,
                boosting_since,
                communication_disabled_until,
                deafened,
                flags,
                muted,
                nickname,
                pending
            ) VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT
                (user_id, guild_id)
            DO UPDATE SET
                guild_id = $2,
                avatar = $3,
                boosting_since = $4,
                communication_disabled_until = $5,
                deafened = $6,
                flags = $7,
                muted = $8,
                nickname = $9,
                pending = $10
            RETURNING
                user_id,
                guild_id,
                avatar as \"avatar: Json<ImageHash>\",
                boosting_since,
                communication_disabled_until,
                deafened,
                flags,
                muted,
                nickname,
                pending
            ",
            member.user.id.get() as i64,
            guild_id,            
            member.avatar.map(Json) as _,
            member.premium_since.map(|x| OffsetDateTime::from_unix_timestamp(x.as_secs()).unwrap()),
            member.communication_disabled_until.map(|x| OffsetDateTime::from_unix_timestamp(x.as_secs()).unwrap()),
            member.deaf,
            member.flags.bits() as i32,
            member.mute,
            member.nick,
            member.pending,
            // member.roles.iter().map(|x| x.get() as i64).collect::<Vec<_>>()
        )
        .fetch_optional(&self.pool)
        .await
    }
}