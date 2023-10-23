use std::fmt::Write;

use std::time::Duration;

use luro_framework::{CommandInteraction, Luro, LuroCommand};
use luro_model::response::LuroResponse;
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
        let response_type = twilight_model::http::interaction::InteractionResponseType::DeferredChannelMessageWithSource;
        ctx.acknowledge_interaction(self.gdpr_export.unwrap_or_default()).await?;
        // The user we are interested in is the interaction author, unless a user was specified
        let user = ctx.get_specified_user_or_author(self.user.as_ref()).await?;
        let user_characters = user.fetch_characters(ctx.database.clone()).await?;

        let mut response = LuroResponse::default();
        let mut embed = ctx.default_embed().await;
        let mut description = String::new();
        let mut timestamp = format!("- Joined discord on <t:{0}> - <t:{0}:R>\n", Duration::from_millis(user.user_id().timestamp().unsigned_abs()).as_secs());

        embed.author(|author| {
            author
                .name(format!("{} - {}", user.name, user.user_id))
                .icon_url(user.avatar())
        }).thumbnail(|t|t.url(user.avatar()));

        if !self.hide_avatar.unwrap_or_default() {
            embed.thumbnail(|thumbnail|thumbnail.url(user.avatar()));

        }
        writeln!(description, "<@{}>", user.user_id)?;
        if let Some(accent_color) = user.accent_colour {
            embed.colour(accent_color as u32);
            writeln!(description, "- Accent Colour: `{accent_color:X}`")?;
        }
        if let Some(email) = &user.email {
            writeln!(description, "- Email: `{}`", email)?;
        }
        if let Some(locale) = &user.locale {
            writeln!(description, "- Locale: `{}`", locale)?;
        }
        if user.mfa_enabled.unwrap_or_default() {
            writeln!(description, "- MFA Enabled: `true`")?;
        }
        if user.system.unwrap_or_default() {
            writeln!(description, "- System Account: `true`")?;
        }
        if user.verified.unwrap_or_default() {
            writeln!(description, " - Verified Account: `true`")?;
        }
        if user.bot {
            writeln!(description, " - Bot: `true`")?;
        }
        if let Some(ref banner) = user.banner() {
            embed.image(|i| i.url(banner));
        }

        // USER DATA SECTION
        let mut user_data_description = String::new();
        {
            if let Some(export) = self.gdpr_export && export {
                if let Some(ref user_specified) = self.user {
                    // TODO: Add privilege esc tally to the person
                    return ctx.respond(|r|r.content(format!("Hey <@{}>! <@{}> is being a cunt and trying to steal your data.", user_specified.resolved.id, ctx.author.user_id())).response_type(response_type)).await
                }
                response.attachments = Some(vec![Attachment::from_bytes(
                    format!("gdpr-export-{}.txt", ctx.author.user_id()),
                    toml::to_string_pretty(&user)?.as_bytes().to_vec(),
                    1
                )]);
            }

            if !user_characters.is_empty() {
                writeln!(user_data_description, "- Has `{}` character profiles", user_characters.len())?;
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

            let marriages = user.fetch_marriages(ctx.database.clone()).await?;
            if !marriages.is_empty() {
                writeln!(user_data_description, "- Has `{}` marriages!", marriages.len())?;
            }

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

            match user_data_description.len() > 1024 {
                true => {
                    writeln!(description, "\n**User Data**\n{user_data_description}")?;
                }
                false => {
                    embed.create_field("User Data", &user_data_description, false);
                }
            }
        }

        if let Some(member) = user.member {
            if let Some(guild) = &ctx.guild {
                let mut role_list = String::new();



                // let user_roles = guild.user_roles(&user);

                // for role in &user_roles {
                //     if role_list.is_empty() {
                //         write!(role_list, "<@&{}>", role.id)?;
                //         continue;
                //     };
                //     write!(role_list, ", <@&{}>", role.id)?
                // }

                // if let Some(role) = user_roles.first() {
                //     if role.colour != 0 {
                //         embed.colour(role.colour);
                //     }
                // }
                // writeln!(guild_information, "- Roles ({}): {role_list}", user_roles.len())?;

                                // embed.create_field(
                //     "Member Permissions",
                //     &format!("```rs\n{:#?}```", guild.user_permission(&user)?),
                //     false,
                // );
            }
                let mut guild_information = String::new();



                timestamp.push_str(format!("- Joined this server at <t:{0}> - <t:{0}:R>\n", member.joined_at()?.as_secs()).as_str());

                // if let Some(left_at) = member.left_as {
                //     timestamp.push_str(format!("- Left this server at <t:{0}> - <t:{0}:R>\n", left_at.as_secs()).as_str());
                // }

                if let Ok(Some(member_timestamp)) = member.boosting_since() {
                    timestamp.push_str(format!("- Boosted this server since <t:{0}> - <t:{0}:R>", member_timestamp.as_secs()).as_str());
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

                match guild_information.len() > 1024 {
                    true => {
                        writeln!(description, "\n**Guild Information**\n{guild_information}")?;
                    }
                    false => {
                        embed.create_field("Guild Information", &guild_information, false);
                    }
                }
        }

        embed.create_field("Timestamps", &timestamp, false);
        embed.create_field("User Database", &user.instance.to_string(), false);
        embed.description(description);
        response.add_embed(embed);
        ctx.response_send(response).await?;
        Ok(())
    }
}
