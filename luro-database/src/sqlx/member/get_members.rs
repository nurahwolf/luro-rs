use sqlx::Error;

use crate::{DbMember, LuroDatabase, LuroUserPermissions};

impl LuroDatabase {
    pub async fn get_members_of_guild(&self, guild_id: i64) -> Result<Vec<DbMember>, Error> {
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
                WHERE gm.guild_id = $1
                GROUP BY u.avatar, avatar_decoration, banner, boosting_since, bot, characters, communication_disabled_until, deafened, discriminator, email, global_name, gm.avatar, gm.user_id, guild_id, joined_at, locale, member_flags, message_edits, messages, mfa_enabled, muted, name, nickname, pending, premium_type, public_flags, system, accent_colour, user_flags, user_permissions, verified, warnings, words_average, words_count
            ",
            guild_id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_guilds_of_member(&self, user_id: i64) -> Result<Vec<DbMember>, Error> {
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
                    gm.guild_id,
                    joined_at,
                    locale,
                    member_flags,
                    message_edits,
                    messages,
                    mfa_enabled,
                    muted,
                    u.name,
                    nickname,
                    pending,
                    premium_type,
                    public_flags,
                    array_agg(gr.role_id) as \"roles: _\",
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
                JOIN guild_member_roles gmr ON gm.user_id = gmr.user_id
                JOIN guild_roles gr ON gmr.role_id = gr.role_id
                WHERE gm.user_id = $1
                GROUP BY u.avatar, avatar_decoration, banner, boosting_since, bot, characters, communication_disabled_until, deafened, discriminator, email, global_name, gm.avatar, gm.user_id, gm.guild_id, joined_at, locale, member_flags, message_edits, messages, mfa_enabled, muted, u.name, nickname, pending, premium_type, public_flags, system, accent_colour, user_flags, user_permissions, verified, warnings, words_average, words_count
                ",
            user_id,
        )
        .fetch_all(&self.pool)
        .await
    }
}
