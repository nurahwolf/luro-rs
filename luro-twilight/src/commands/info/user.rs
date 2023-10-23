use std::fmt::Write;

use std::time::Duration;

use luro_database::LuroUser;
use luro_framework::{CommandInteraction, Luro, LuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::{
    http::attachment::Attachment,
    id::{marker::GenericMarker, Id},
};
use twilight_util::snowflake::Snowflake;

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "user", desc = "Information about a user")]
pub struct InfoUser {
    /// The user to get, gets yourself if not specified
    user: Option<ResolvedUser>,
    /// Optionally try to get a user from a different guild
    guild: Option<Id<GenericMarker>>,
    /// Hide the user's avatar, so that there is more space for the details
    hide_avatar: Option<bool>,
    /// Set this if you want a copy of your data.
    gdpr_export: Option<bool>,
}

impl LuroCommand for InfoUser {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        // Fetch the user requested. Additional check for if we want another guild.
        let mut user = ctx.get_specified_user_or_author(self.user.as_ref(), true).await?;
        if let Some(guild_id) = self.guild {
            user = LuroUser::new(ctx.database.clone(), user.user_id(), Some(guild_id.cast()), true).await?
        }

        // Base embed
        let mut embed = ctx.default_embed().await;
        embed
            .author(|author| {
                author
                    .name(format!("Infomation on {} | {}", user.name, user.user_id))
                    .icon_url(user.avatar_url())
            })
            .thumbnail(|t| t.url(user.avatar_url()));
        if !self.hide_avatar.unwrap_or_default() {
            embed.thumbnail(|thumbnail| thumbnail.url(user.avatar_url()));
        }
        if let Some(ref banner) = user.banner_url() {
            embed.image(|i| i.url(banner));
        }

        let mut timestamp = format!(
            "- Joined discord on <t:{0}> - <t:{0}:R>\n",
            Duration::from_millis(user.user_id().timestamp().unsigned_abs()).as_secs()
        );

        // Luro data
        if let Some(ref user_data) = user.data {
            let mut luro_information = String::new();
            writeln!(luro_information, "- Is marked as `{}` in my database!", user_data.permissions)?;

            if let Ok(user_characters) = user.fetch_characters(ctx.database.clone()).await {
                if !user_characters.is_empty() {
                    writeln!(luro_information, "- Has `{}` character profiles", user_characters.len())?;
                }
            }

            let marriages = user.fetch_marriages(ctx.database.clone()).await?;
            if !marriages.is_empty() {
                writeln!(luro_information, "- Has `{}` marriages!", marriages.len())?;
            }

            // writeln!(user_data_description, "- Typed `{}` characters", user.averagesize)?;
            // writeln!(
            //     user_data_description,
            //     "- Has said `{}` words with an average length of `{}` characters per word",
            //     user.wordcount,
            //     user.averagesize.checked_div(user.wordcount).unwrap_or(0)
            // )?;

            // if user.moderation_actions_performed != 0 {
            //     writeln!(
            //         user_data_description,
            //         "- Performed `{}` moderation actions",
            //         user.moderation_actions_performed
            //     )?;
            // }

            // if user.message_edits != 0 {
            //     writeln!(user_data_description, "- Edited `{}` messages", user.message_edits)?;
            // }

            // if !user.moderation_actions.is_empty() {
            //     writeln!(
            //         user_data_description,
            //         "**Received `{}` punishments**",
            //         user.moderation_actions.len()
            //     )?;
            //     let (mut bans, mut kicks, mut warnings, mut priv_esc) = (0, 0, 0, 0);
            //     for punishment in &user.moderation_actions {
            //         for punishment_type in &punishment.action_type {
            //             match punishment_type {
            //                 UserActionType::Ban => bans += 1,
            //                 UserActionType::Kick => kicks += 1,
            //                 UserActionType::Warn => warnings += 1,
            //                 UserActionType::PrivilegeEscalation => priv_esc += 1,
            //                 _ => (),
            //             }
            //         }
            //     }
            //     if bans != 0 {
            //         writeln!(user_data_description, "- Banned `{bans}` times")?;
            //     }
            //     if kicks != 0 {
            //         writeln!(user_data_description, "- Kicked `{kicks}` times")?;
            //     }
            //     if priv_esc != 0 {
            //         writeln!(user_data_description, "- Attempted Privilege Escalation `{priv_esc}` times")?;
            //     }
            //     if warnings != 0 {
            //         writeln!(user_data_description, "- Warned *(including expired)* `{warnings}` times")?;
            //     }
            // }

            // if !user.warnings.is_empty() {
            //     writeln!(user_data_description, "- Has `{}` active warnings", user.warnings.len())?;
            // }

            embed.create_field("Luro Information", &luro_information, false);
        }

        // Member only information
        match user.member {
            Some(ref member) => {
                let mut guild_information = String::new();
                let mut role_list = String::new();

                timestamp.push_str(format!("- Joined this server at <t:{0}> - <t:{0}:R>\n", member.joined_at.unix_timestamp()).as_str());
                embed.description(format!("Hiya <@{}>! This is what I know about you.\n\nLooks like I was able to map you to the guild <#{}>, so I'm showing you information related to there.", user.user_id, member.guild_id));

                if let Some(left_at) = member.left_at {
                    timestamp.push_str(format!("- Left this server at <t:{0}> - <t:{0}:R>\n", left_at.unix_timestamp()).as_str());
                }
                if let Some(member_timestamp) = member.boosting_since {
                    timestamp
                        .push_str(format!("- Boosted this server since <t:{0}> - <t:{0}:R>", member_timestamp.unix_timestamp()).as_str());
                }
                if let Some(nickname) = &member.nickname {
                    writeln!(guild_information, "- Nickname: `{nickname}`")?;
                }
                if member.deafened {
                    writeln!(guild_information, "- Deafened: `true`")?;
                }
                if member.muted {
                    writeln!(guild_information, "- Muted: `true`")?;
                }
                if member.pending {
                    writeln!(guild_information, "- Pending: `true`")?;
                }
                if let Ok(Some(timestamp)) = member.communication_disabled_until() {
                    writeln!(guild_information, "- Timed out until: <t:{}:R>", timestamp.as_secs())?;
                }

                // TODO: Once member_banner is a thing in [Member]
                // if let Some(banner) = get_member_banner(&member, guild_id, user) {
                //     embed = embed.image(ImageSource::url(banner)?)
                // }

                for role_id in member.sorted_roles() {
                    if role_list.is_empty() {
                        write!(role_list, "<@&{role_id}>")?;
                        continue;
                    };
                    write!(role_list, ", <@&{role_id}>")?
                }

                writeln!(guild_information, "- Roles ({}): {role_list}", member.roles.len())?;
                embed.create_field("Guild Information", &guild_information, false);
                if let Ok(member_permissions) = member.permission_calculator(ctx.database.clone(), &member.role_permissions()).await {
                    embed.create_field("Guild Permissions", &format!("```rs\n{:#?}```", member_permissions.root()), false);
                    // TODO: Complete this
                }
            }
            None => {
                embed.description(format!("Hiya <@{}>! This is what I know about you.", user.user_id));
            }
        }

        // Standard user only information
        let mut user_information = String::new();
        if let Some(accent_color) = user.accent_colour {
            embed.colour(accent_color as u32);
            writeln!(user_information, "- Accent Colour: `{accent_color:X}`")?;
        }
        if let Some(email) = &user.email {
            writeln!(user_information, "- Email: `{}`", email)?;
        }
        if let Some(locale) = &user.locale {
            writeln!(user_information, "- Locale: `{}`", locale)?;
        }
        if user.mfa_enabled.unwrap_or_default() {
            writeln!(user_information, "- MFA Enabled: `true`")?;
        }
        if user.system.unwrap_or_default() {
            writeln!(user_information, "- System Account: `true`")?;
        }
        if user.verified.unwrap_or_default() {
            writeln!(user_information, " - Verified Account: `true`")?;
        }
        if user.bot {
            writeln!(user_information, " - Bot: `true`")?;
        }
        embed.create_field("User Information", &user_information, false);
        embed.create_field("Timestamps", &timestamp, false);
        embed.create_field(
            "Data Source",
            match user.instance {
                luro_database::LuroUserType::User => "Twilight User - Data fetched using the Discord API",
                luro_database::LuroUserType::Member => "Twilight Member - Data fetched using the Discord API, including guild data",
                luro_database::LuroUserType::DbUser => "Luro User - Data fetched from my database only, with includes your custom stuff!",
                luro_database::LuroUserType::DbMember => "Luro Member - Data fetched from my database, including guild information!",
            },
            false,
        );

        ctx.respond(|response| {
            // Handle attempts at stealing data
            if self.gdpr_export.unwrap_or_default() {
                if self.user.is_some() {
                    // TODO: Add privilege esc tally to the person
                    response.set_embeds(vec![]);
                    response.content(format!(
                        "Hey <@{}>! <@{}> is being a cunt and trying to steal your data.",
                        user.user_id(),
                        ctx.author.user_id()
                    ));
                } else {
                    response.ephemeral().attachments = Some(vec![Attachment::from_bytes(
                        format!("gdpr-export-{}.txt", ctx.author.user_id()),
                        toml::to_string_pretty(&user).unwrap().as_bytes().to_vec(), // TODO: Handle this unwrap
                        1,
                    )]);
                }
            }
            response.add_embed(embed)
        })
        .await
    }
}
