use sqlx::Error;

use crate::{LuroDatabase, LuroUser};

impl LuroDatabase {
    pub async fn get_members_of_guild(&self, guild_id: i64) -> Result<Vec<LuroUser>, Error> {
        let mut users = vec![];
        let result = sqlx::query_file!("queries/guild_members/get_guild_members.sql", guild_id)
            .fetch_all(&self.pool)
            .await?;

        for user in result {
            if let Ok(Some(user)) = self.get_member(user.user_id, user.guild_id).await {
                users.push(user)
            }
        }

        Ok(users)
    }

    // pub async fn get_guilds_of_member(&self, user_id: i64) -> Result<Vec<DbMember>, Error> {
    //     sqlx::query_as!(
    //         DbMember,
    //         "
    //             SELECT
    //                 accent_colour,
    //                 avatar_decoration,
    //                 banner,
    //                 boosting_since,
    //                 bot,
    //                 characters,
    //                 communication_disabled_until,
    //                 deafened,
    //                 discriminator,
    //                 email,
    //                 global_name,
    //                 guild_avatar,
    //                 u.user_id,
    //                 gm.guild_id,
    //                 joined_at,
    //                 locale,
    //                 member_flags,
    //                 message_edits,
    //                 messages,
    //                 mfa_enabled,
    //                 muted,
    //                 u.name,
    //                 nickname,
    //                 pending,
    //                 premium_type,
    //                 public_flags,
    //                 array_agg(gr.role_id) as \"roles: _\",
    //                 system,
    //                 user_avatar,
    //                 user_flags,
    //                 user_permissions as \"user_permissions: LuroUserPermissions\",
    //                 verified,
    //                 warnings,
    //                 words_average,
    //                 words_count
    //             FROM guild_members gm
    //             JOIN users u ON gm.user_id = u.user_id
    //             JOIN guild_member_roles gmr ON gm.user_id = gmr.user_id
    //             JOIN guild_roles gr ON gmr.role_id = gr.role_id
    //             WHERE gm.user_id = $1
    //             GROUP BY user_avatar, avatar_decoration, banner, boosting_since, bot, characters, communication_disabled_until, deafened, discriminator, email, global_name, guild_avatar, u.user_id, gm.guild_id, joined_at, locale, member_flags, message_edits, messages, mfa_enabled, muted, name, nickname, pending, premium_type, public_flags, system, accent_colour, user_flags, user_permissions, verified, warnings, words_average, words_count                ",
    //         user_id,
    //     )
    //     .fetch_all(&self.pool)
    //     .await
    // }
}
