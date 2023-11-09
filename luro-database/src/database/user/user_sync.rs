use luro_model::types::User;

use crate::Database;

impl Database {
    /// By default, data is returned from the database where possible. Because of this, it might be a little outdated.
    /// Calling Sync will use the passed twilight client to make a call to the Discord API, ensuring we have fresh data.
    /// The result is then written back to the database.
    ///
    /// NOTE: This syncs based on the data currently present.
    /// e.g. If there is no [LuroUserData] then Luro specific data is not synced
    ///
    /// Likewise, if there is no [LuroMember] then guild data is not synced.
    ///
    /// NOTE: This command does NOT fail, errors are raised to the console and data is simply not written to the database if unsuccessful.
    ///
    /// This also means if we can't update a user with new data from the API, such as no longer being in that guild, that no new data will be returned.
    pub async fn user_sync<'a>(&'a self, user: &'a mut User) -> &mut User {
        // Sync Luro specific data, if present
        if let Some(data) = user.data.clone() {
            if let Err(why) = self.driver.update_user_data(user.user_id, data).await {
                tracing::warn!(why = ?why, "Failed to sync luro user data with the database")
            }
        }

        // If a member context, sync member and return
        if let Some(ref mut member) = user.member {
            match self.api_client.guild_member(member.guild_id, user.user_id).await {
                Ok(twilight_member) => match twilight_member.model().await {
                    Ok(twilight_member) => {
                        member.avatar = twilight_member.avatar;
                        member.boosting_since = twilight_member.premium_since;
                        member.communication_disabled_until = twilight_member.communication_disabled_until;
                        member.deafened = twilight_member.deaf;
                        member.flags = twilight_member.flags;
                        member.joined_at = twilight_member.joined_at;
                        member.muted = twilight_member.mute;
                        member.nickname = twilight_member.nick.clone();
                        member.pending = twilight_member.pending;
                        member.roles = twilight_member.roles.clone();
                        member.user_id = user.user_id;

                        user.accent_colour = twilight_member.user.accent_color;
                        user.avatar_decoration = twilight_member.user.avatar_decoration;
                        user.avatar = twilight_member.user.avatar;
                        user.banner = twilight_member.user.banner;
                        user.bot = twilight_member.user.bot;
                        user.discriminator = twilight_member.user.discriminator;
                        user.email = twilight_member.user.email.clone();
                        user.flags = twilight_member.user.flags;
                        user.global_name = twilight_member.user.global_name.clone();
                        user.locale = twilight_member.user.locale.clone();
                        user.mfa_enabled = twilight_member.user.mfa_enabled;
                        user.name = twilight_member.user.name.clone();
                        user.premium_type = twilight_member.user.premium_type;
                        user.public_flags = twilight_member.user.public_flags;
                        user.system = twilight_member.user.system;
                        user.verified = twilight_member.user.verified;
                        user.user_id = twilight_member.user.id;

                        if let Some(ref data) = member.data {
                            if let Err(why) = self.driver.update_member_data(data).await {
                                tracing::warn!(why = ?why, "Failed to sync luro member data with the database")
                            }
                        }

                        if let Err(why) = self.driver.update_member((member.guild_id, &twilight_member)).await {
                            tracing::warn!(why = ?why, "Failed to sync twilight member with the database")
                        }
                    }
                    Err(why) => tracing::error!(why = ?why, "Failed to convert received data from API into a Twilight user"),
                },
                Err(why) => tracing::error!(why = ?why, "Failed to fetch member using Twilight API"),
            }

            return user;
        }

        // Not a member, so sync user
        match self.api_client.user(user.user_id).await {
            Ok(twilight_user) => match twilight_user.model().await {
                Ok(twilight_user) => {
                    user.accent_colour = twilight_user.accent_color;
                    user.avatar_decoration = twilight_user.avatar_decoration;
                    user.avatar = twilight_user.avatar;
                    user.banner = twilight_user.banner;
                    user.bot = twilight_user.bot;
                    user.discriminator = twilight_user.discriminator;
                    user.email = twilight_user.email.clone();
                    user.flags = twilight_user.flags;
                    user.global_name = twilight_user.global_name.clone();
                    user.locale = twilight_user.locale.clone();
                    user.mfa_enabled = twilight_user.mfa_enabled;
                    user.name = twilight_user.name.clone();
                    user.premium_type = twilight_user.premium_type;
                    user.public_flags = twilight_user.public_flags;
                    user.system = twilight_user.system;
                    user.verified = twilight_user.verified;
                    user.user_id = twilight_user.id;

                    if let Err(why) = self.driver.update_user(&twilight_user).await {
                        tracing::warn!(why = ?why, "Failed to sync twilight user with the database")
                    }
                }
                Err(why) => tracing::error!(why = ?why, "Failed to convert received data from API into a Twilight user"),
            },
            Err(why) => tracing::error!(why = ?why, "Failed to fetch user using Twilight API"),
        }

        user
    }
}
