use sqlx::Error;
use twilight_model::id::{
    marker::{GuildMarker, UserMarker},
    Id,
};

use crate::{DbMember, LuroDatabase, LuroUserPermissions};

impl LuroDatabase {
    pub async fn get_member(&self, user_id: &Id<UserMarker>, guild_id: &Id<GuildMarker>) -> Result<Option<DbMember>, Error> {
        sqlx::query_as!(
            DbMember,
            "
                SELECT
                    accent_colour,
                    avatar_decoration,
                    banner,
                    boosting_since,
                    bot,
                    characters,
                    communication_disabled_until,
                    deafened,
                    discriminator,
                    email,
                    global_name,
                    gm.avatar as \"guild_avatar: String\",
                    gm.user_id as \"user_id: i64\",
                    guild_id,
                    joined_at,
                    locale,
                    member_flags,
                    message_edits,
                    messages,
                    mfa_enabled,
                    muted,
                    name,
                    nickname,
                    pending,
                    premium_type,
                    public_flags,
                    array_agg(role_id) as roles,
                    system,
                    u.avatar as \"avatar: String\",
                    user_flags,
                    user_permissions as \"user_permissions: LuroUserPermissions\",
                    verified,
                    warnings,
                    words_average,
                    words_count
                FROM guild_members gm
                JOIN users u ON gm.user_id = u.user_id
                JOIN guild_member_roles r ON gm.user_id = r.user_id
                WHERE
                    u.user_id = $1
                        and
                    gm.guild_id = $2
                GROUP BY u.avatar, avatar_decoration, banner, boosting_since, bot, characters, communication_disabled_until, deafened, discriminator, email, global_name, gm.avatar, gm.user_id, guild_id, joined_at, locale, member_flags, message_edits, messages, mfa_enabled, muted, name, nickname, pending, premium_type, public_flags, system, accent_colour, user_flags, user_permissions, verified, warnings, words_average, words_count
            ",
            user_id.get() as i64,
            guild_id.get() as i64
        )
        .fetch_optional(&self.pool)
        .await
    }
}
