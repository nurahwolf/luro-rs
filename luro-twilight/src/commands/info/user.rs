use std::fmt::Write;

use std::time::Duration;

use luro_framework::{CommandInteraction, Luro, LuroCommand};
use luro_model::{response::LuroResponse, user::actions_type::UserActionType};
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::{
    http::{attachment::Attachment, interaction::InteractionResponseType},
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
        ctx.acknowledge_interaction(self.gdpr_export.unwrap_or_default()).await?;
        // The user we are interested in is the interaction author, unless a user was specified
        let mut user = ctx.get_specified_user_or_author(self.user.as_ref()).await?;
        let mut embed = ctx.default_embed().await;

        // Added information for if we are in a guild
        if let Some(guild_id) = ctx.guild_id {
            let member = ctx.get_member(&user.user_id(), &guild_id).await?;
            let member_avatar = member.avatar(&ctx.database).await?;
            embed.thumbnail(|t|t.url(member_avatar));
        } else {
            embed.thumbnail(|t|t.url(user.avatar()));
        }






        let user_timestamp = Duration::from_millis(user.user_id.timestamp().unsigned_abs());

        let mut response = LuroResponse::default();
        let mut embed = ctx.default_embed().await;
        let mut description = String::new();
        let mut timestamp = format!("- Joined discord on <t:{0}> - <t:{0}:R>\n", user_timestamp.as_secs());

        embed.author(|author| {
            author
                .name(format!("{} - {}", user.name, user.user_id))
                .icon_url(user.avatar())
        });
        if let Some(hide_avatar) = self.hide_avatar && hide_avatar {
        } else {
            embed.thumbnail(|thumbnail|thumbnail.url(user.avatar()));
        }

        writeln!(description, "<@{}>", user.id)?;
        if let Some(accent_color) = user.accent_color {
            embed.colour(accent_color);
            writeln!(description, "- Accent Colour: `{accent_color:X}`")?;
        }
        if let Some(email) = &user.email {
            writeln!(description, "- Email: `{}`", email)?;
        }
        if let Some(locale) = &user.locale {
            writeln!(description, "- Locale: `{}`", locale)?;
        }
        if user.mfa_enabled {
            writeln!(description, "- MFA Enabled: `true`")?;
        }
        if user.system {
            writeln!(description, "- System Account: `true`")?;
        }
        if user.verified {
            writeln!(description, " - Verified Account: `true`")?;
        }
        if user.bot {
            writeln!(description, " - Bot: `true`")?;
        }
        if let Some(ref banner) = user.banner() {
            embed.image(|i| i.url(banner));
        }

        // Some additional details if we are a guild
        let guild_id = match self.guild {
            Some(guild_specified) => Some(Id::new(guild_specified.get())),
            None => ctx.guild_id,
        };

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

            if !user.characters.is_empty() {
                writeln!(user_data_description, "- Has `{}` character profiles", user.characters.len())?;
            }
            writeln!(user_data_description, "- Typed `{}` characters", user.averagesize)?;
            writeln!(
                user_data_description,
                "- Has said `{}` words with an average length of `{}` characters per word",
                user.wordcount,
                user.averagesize.checked_div(user.wordcount).unwrap_or(0)
            )?;

            if user.moderation_actions_performed != 0 {
                writeln!(
                    user_data_description,
                    "- Performed `{}` moderation actions",
                    user.moderation_actions_performed
                )?;
            }

            if user.message_edits != 0 {
                writeln!(user_data_description, "- Edited `{}` messages", user.message_edits)?;
            }

            if !user.marriages.is_empty() {
                writeln!(user_data_description, "- Has `{}` marriages!", user.marriages.len())?;
            }

            if !user.moderation_actions.is_empty() {
                writeln!(
                    user_data_description,
                    "**Received `{}` punishments**",
                    user.moderation_actions.len()
                )?;
                let (mut bans, mut kicks, mut warnings, mut priv_esc) = (0, 0, 0, 0);
                for punishment in &user.moderation_actions {
                    for punishment_type in &punishment.action_type {
                        match punishment_type {
                            UserActionType::Ban => bans += 1,
                            UserActionType::Kick => kicks += 1,
                            UserActionType::Warn => warnings += 1,
                            UserActionType::PrivilegeEscalation => priv_esc += 1,
                            _ => (),
                        }
                    }
                }
                if bans != 0 {
                    writeln!(user_data_description, "- Banned `{bans}` times")?;
                }
                if kicks != 0 {
                    writeln!(user_data_description, "- Kicked `{kicks}` times")?;
                }
                if priv_esc != 0 {
                    writeln!(user_data_description, "- Attempted Privilege Escalation `{priv_esc}` times")?;
                }
                if warnings != 0 {
                    writeln!(user_data_description, "- Warned *(including expired)* `{warnings}` times")?;
                }
            }

            if !user.warnings.is_empty() {
                writeln!(user_data_description, "- Has `{}` active warnings", user.warnings.len())?;
            }

            match user_data_description.len() > 1024 {
                true => {
                    writeln!(description, "\n**User Data**\n{user_data_description}")?;
                }
                false => {
                    embed.create_field("User Data", &user_data_description, false);
                }
            }
        }

        if let Some(guild_id) = guild_id {
            let guild = ctx.get_guild(&guild_id).await?;
            if let Ok(guild_member) = ctx.twilight_client.guild_member(guild_id, user.id).await {
                user.update_member(&guild_id, &guild_member.model().await?);
            }

            if let Some(luro_member) = user.guilds.get(&guild_id) {
                let mut guild_information = String::new();
                let mut role_list = String::new();

                let user_roles = guild.user_roles(&user);
                ctx.database.update_user(user.clone()).await?;

                for role in &user_roles {
                    if role_list.is_empty() {
                        write!(role_list, "<@&{}>", role.id)?;
                        continue;
                    };
                    write!(role_list, ", <@&{}>", role.id)?
                }

                if let Some(role) = user_roles.first() {
                    if role.colour != 0 {
                        embed.colour(role.colour);
                    }
                }
                writeln!(guild_information, "- Roles ({}): {role_list}", user_roles.len())?;
                timestamp.push_str(format!("- Joined this server at <t:{0}> - <t:{0}:R>\n", luro_member.joined_at.as_secs()).as_str());
                if let Some(left_at) = luro_member.left_at {
                    timestamp.push_str(format!("- Left this server at <t:{0}> - <t:{0}:R>\n", left_at.as_secs()).as_str());
                }

                if let Some(member_timestamp) = luro_member.premium_since {
                    timestamp.push_str(format!("- Boosted this server since <t:{0}> - <t:{0}:R>", member_timestamp.as_secs()).as_str());
                }
                if let Some(nickname) = &luro_member.nick {
                    writeln!(guild_information, "- Nickname: `{nickname}`")?;
                }
                if luro_member.deaf {
                    writeln!(guild_information, "- Deafened: `true`")?;
                }
                if luro_member.mute {
                    writeln!(guild_information, "- Muted: `true`")?;
                }
                if luro_member.pending {
                    writeln!(guild_information, "- Pending: `true`")?;
                }
                if let Some(timestamp) = luro_member.communication_disabled_until {
                    writeln!(guild_information, "- Timed out until: <t:{}:R>", timestamp.as_secs())?;
                }

                // TODO: Once member_banner is a thing in [Member]
                // if let Some(banner) = get_member_banner(&member, guild_id, user) {
                //     embed = embed.image(ImageSource::url(banner)?)
                // }
                embed.author(|author| {
                    author
                        .name(user.member_name(&Some(guild_id)))
                        .icon_url(user.guild_avatar(&guild_id))
                });
                match guild_information.len() > 1024 {
                    true => {
                        writeln!(description, "\n**Guild Information**\n{guild_information}")?;
                    }
                    false => {
                        embed.create_field("Guild Information", &guild_information, false);
                    }
                }
                // embed.create_field(
                //     "Member Permissions",
                //     &format!("```rs\n{:#?}```", guild.user_permission(&user)?),
                //     false,
                // );
            }
        }

        embed.create_field("Timestamps", &timestamp, true);
        embed.description(description);
        response.add_embed(embed);
        ctx.response_send(response).await?;
        Ok(())
    }
}
