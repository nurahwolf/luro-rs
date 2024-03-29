use std::{sync::Arc, time::Duration};

use anyhow::Context;
use luro_database::Database;
use luro_framework::{
    CommandInteraction, ComponentInteraction, Luro, {CreateLuroCommand, LuroCommand},
};
use luro_model::{
    builders::{ComponentBuilder, EmbedBuilder},
    response::SimpleResponse,
    types::{Member, User},
};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    channel::message::component::ButtonStyle,
    http::interaction::InteractionResponseType,
    id::{marker::GuildMarker, Id},
};
use twilight_util::snowflake::Snowflake;

mod database;
mod guild;
// mod punishments;
mod message;
mod role;
mod user;

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "info", desc = "Information about neat things")]
pub enum Info {
    #[command(name = "user")]
    User(user::InfoUser),
    #[command(name = "role")]
    Role(role::InfoRole),
    #[command(name = "guild")]
    Guild(guild::Guild),
    #[command(name = "message")]
    Message(message::Message),
    // #[command(name = "punishments")]
    // Punishments(punishments::Punishments),
    #[command(name = "database")]
    Database(database::Database),
}

impl CreateLuroCommand for Info {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        match self {
            Self::Guild(command) => command.interaction_command(ctx).await,
            // Self::Punishments(command) => command.interaction_command(ctx).await,
            Self::Role(command) => command.interaction_command(ctx).await,
            Self::Message(command) => command.interaction_command(ctx).await,
            Self::User(command) => command.interaction_command(ctx).await,
            Self::Database(command) => command.interaction_command(ctx).await,
        }
    }

    async fn interaction_component(
        self,
        ctx: ComponentInteraction,
        original_interaction: twilight_model::application::interaction::Interaction,
    ) -> anyhow::Result<luro_model::types::CommandResponse> {
        let mut user = match &self {
            Info::User(user_command) => match &user_command.user {
                Some(user) => ctx.fetch_user(*user).await?,
                None => ctx.fetch_user(original_interaction.author_id().context("Expected user")?).await?,
            },
            _ => ctx.fetch_user(original_interaction.author_id().context("Expected user")?).await?,
        };
        let mut embed: EmbedBuilder = ctx
            .message
            .embeds
            .first()
            .context("Expected there to be an embed attached to this message. Do I have permission to see this channel?")?
            .clone()
            .into();

        let embed = match ctx.command_name() {
            "info-button-guild-permissions" => info_button_guild_permissions(&ctx, user, &mut embed).await?,
            "info-button-timestamps" => timestamps(&ctx.author, &user, &mut embed),
            "info-button-messages" => return info_recent_messages(&ctx, user).await,
            "info-button-luro" => luro_information(&ctx.author, &user, ctx.database.clone(), &mut embed).await,
            "info-button-user" => user_information(&ctx.author, &user, &mut embed),
            "info-button-guild" => guild_information(&ctx.author, &user.member.context("Expected to get member data")?, &mut embed),
            "info-button-clear" => embed.set_fields(vec![]),
            "info-button-sync" => sync(&ctx, &mut user, &mut embed).await?,
            name => return ctx.simple_response(SimpleResponse::UnknownCommand(name)).await,
        };

        ctx.respond(|r| {
            r.response_type(InteractionResponseType::UpdateMessage)
                .set_embed(embed.0.clone())
                .components(|c| {
                    *c = buttons(ctx.guild_id(), true);
                    c
                })
        })
        .await
    }
}

pub async fn sync<'a>(ctx: &ComponentInteraction, user: &mut User, embed: &'a mut EmbedBuilder) -> anyhow::Result<&'a mut EmbedBuilder> {
    ctx.database.user_sync(user).await;

    for field in embed.0.fields.iter_mut() {
        if field.name.contains("Timestamps") {
            let mut timestamp = format!(
                "- Joined discord on <t:{0}> - <t:{0}:R>\n",
                Duration::from_millis(user.user_id.timestamp().unsigned_abs()).as_secs()
            );

            if let Some(member) = &user.member {
                timestamp.push_str(format!("- Joined this server at <t:{0}> - <t:{0}:R>\n", member.joined_at.as_secs()).as_str());

                if let Some(member_timestamp) = member.boosting_since {
                    timestamp.push_str(format!("- Boosted this server since <t:{0}> - <t:{0}:R>\n", member_timestamp.as_secs()).as_str());
                }
                if let Some(ref data) = member.data {
                    if let Some(left_at) = data.left_at {
                        timestamp.push_str(format!("- Left this server at <t:{0}> - <t:{0}:R>\n", left_at.unix_timestamp()).as_str());
                    }
                }
            }

            field.value = timestamp;
        }

        if field.name.contains("Guild-Level Permissions") {
            if let Some(ref member) = user.member {
                if let Some(ref data) = member.data {
                    let permissions = data.role_permissions();
                    let member_permissions = data.permission_calculator(&permissions);
                    let mut permissions = vec![];
                    for (permission, _) in member_permissions.root().iter_names() {
                        permissions.push(permission)
                    }

                    permissions.sort();
                    field.value = format!("```\n{}```", permissions.join(" | "));
                }
            }
        }

        if field.name.contains("Luro Information") {
            let mut luro_information = String::new();

            if let Ok(user_characters) = ctx.database.user_fetch_characters(user.user_id).await {
                if !user_characters.is_empty() {
                    luro_information.push_str(&format!("- Has `{}` character profiles\n", user_characters.len()));
                }
            }

            if let Ok(marriages) = ctx.database.user_fetch_marriages(user.user_id).await {
                let mut active_marriages = 0;

                for marriage in marriages {
                    if !marriage.divorced && !marriage.rejected {
                        active_marriages += 1;
                    }
                }

                if active_marriages != 0 {
                    luro_information.push_str(&format!("- Has `{active_marriages}` marriages\n"));
                }
            }

            if let Some(ref user_data) = user.data {
                luro_information.push_str(&format!("- Is marked as `{}` in my database!\n", user_data.permissions));

                if let Some(gender) = &user_data.gender
                    && let Some(sexuality) = &user_data.sexuality
                {
                    luro_information.push_str(&format!("- Has a sexuality of `{sexuality}` and identifies as `{gender}`\n"));
                } else if let Some(gender) = &user_data.gender {
                    luro_information.push_str(&format!("- Identifies as `{gender}`\n"));
                } else if let Some(sexuality) = &user_data.sexuality {
                    luro_information.push_str(&format!("- Has a sexuality of `{sexuality}`\n"));
                }
            }
            if let Ok(word_count) = ctx.database.user_count_messages(user.user_id).await
                && word_count.total_messages.unwrap_or_default() != 0
            {
                if let Some(count) = word_count.total_messages
                    && count != 0
                {
                    luro_information.push_str(&format!("- Has sent `{count}` messages!\n"))
                };
                if let Some(count) = word_count.total_words
                    && count != 0
                {
                    luro_information.push_str(&format!("  - `{count}` words said!\n"))
                };
                if let Some(count) = word_count.total_unique_words
                    && count != 0
                {
                    luro_information.push_str(&format!("  - `{count}` unique words said!\n"))
                };
                if let Some(count) = word_count.total_custom_messages
                    && count != 0
                {
                    luro_information.push_str(&format!("  - `{count}` custom messages\n"))
                };
                if let Some(count) = word_count.total_message_creates
                    && count != 0
                {
                    luro_information.push_str(&format!("  - `{count}` messages created\n"))
                };
                if let Some(count) = word_count.total_message_cached
                    && count != 0
                {
                    luro_information.push_str(&format!("  - `{count}` messages cached\n"))
                };
                if let Some(count) = word_count.total_message_deletes
                    && count != 0
                {
                    luro_information.push_str(&format!("  - `{count}` messages deleted\n"))
                };
                if let Some(count) = word_count.total_message_updates
                    && count != 0
                {
                    luro_information.push_str(&format!("  - `{count}` messages updated\n"))
                };
                if let Some(count) = word_count.total_message_message
                    && count != 0
                {
                    luro_information.push_str(&format!("  - `{count}` messages stored\n"))
                };
            }

            field.value = luro_information;
        }

        if field.name.contains("User Information") {
            let mut user_information = String::new();
            if let Some(flags) = &user.flags
                && !flags.is_empty()
            {
                tracing::info!("flags - {}", flags.bits());
                if flags.bits() == 1 << 20 {
                    user_information.push_str("**USER IS MARKED FOR UNUSUAL AMOUNTS OF DMS**")
                }

                let mut flags_sorted = vec![];
                for (flag, _) in flags.iter_names() {
                    flags_sorted.push(flag)
                }
                flags_sorted.sort();
                user_information.push_str(&format!(
                    "\n- User Flags (`{}`): \n```\n{}```",
                    flags.bits(),
                    flags_sorted.join(" | ")
                ));
            }

            if let Some(flags) = &user.public_flags
                && !flags.is_empty()
            {
                if flags.bits() == 1 << 20 {
                    user_information.push_str("**USER IS MARKED FOR UNUSUAL AMOUNTS OF DMS**")
                }

                let mut flags_sorted = vec![];
                for (flag, _) in flags.iter_names() {
                    flags_sorted.push(flag)
                }
                flags_sorted.sort();
                user_information.push_str(&format!(
                    "\n- Public Flags (`{}`): \n```\n{}```",
                    flags.bits(),
                    flags_sorted.join(" | ")
                ))
            }

            if let Some(accent_colour) = user.accent_colour {
                user_information.push_str(&format!("\n- Accent Colour: `{accent_colour:X}`"));
            } else if let Some(member) = &user.member {
                if let Some(data) = &member.data {
                    if let Some(role) = data.highest_role_colour() {
                        user_information.push_str(&format!("\n- Accent Colour: `{:X}` (<@&{}>)", role.colour, role));
                    }
                }
            }
            if let Some(email) = &user.email {
                user_information.push_str(&format!("\n- Email: `{}`", email));
            }
            if let Some(locale) = &user.locale {
                user_information.push_str(&format!("\n- Locale: `{}`", locale));
            }
            if user.mfa_enabled.unwrap_or_default() {
                user_information.push_str("\n- MFA Enabled: `true`");
            }
            if user.system.unwrap_or_default() {
                user_information.push_str("\n- System Account: `true`");
            }
            if user.verified.unwrap_or_default() {
                user_information.push_str("\n- Verified Account: `true`");
            }
            if user.bot {
                user_information.push_str("\n- Bot: `true`");
            }

            match user_information.is_empty() {
                true => field.value = "- I don't have any user information to report!".to_owned(),
                false => field.value = user_information,
            };
        }

        if field.name.contains("Guild Information") {
            if let Some(member) = user.member.clone() {
                let mut guild_information = String::new();
                let mut role_list = String::new();

                if let Some(nickname) = &member.nickname {
                    guild_information.push_str(&format!("\n- Nickname: `{nickname}`"));
                }
                if member.deafened {
                    guild_information.push_str("\n- Deafened: `true`");
                }
                if member.muted {
                    guild_information.push_str("\n- Muted: `true`");
                }
                if member.pending {
                    guild_information.push_str("\n- Pending: `true`");
                }

                let mut flags_sorted = vec![];
                for (flag, _) in member.flags.iter_names() {
                    flags_sorted.push(flag)
                }
                flags_sorted.sort();
                if !flags_sorted.is_empty() {
                    guild_information.push_str(&format!(
                        "\n- Member Flags (`{}`): \n```\n{}```",
                        member.flags.bits(),
                        flags_sorted.join(" | ")
                    ));
                }

                // TODO: Once member_banner is a thing in [Member]
                // if let Some(banner) = get_member_banner(&member, guild_id, user) {
                //     embed = embed.image(ImageSource::url(banner)?)
                // }

                if let Some(ref data) = member.data {
                    if data.guild_owner {
                        guild_information.push_str("\n- Is the owner of this guild!");
                    }

                    for role_id in data.sorted_roles() {
                        if role_list.is_empty() {
                            role_list.push_str(&format!("<@&{role_id}>"));
                            continue;
                        };
                        role_list.push_str(&format!(", <@&{role_id}>"));
                    }

                    if !role_list.is_empty() {
                        guild_information.push_str(&format!("\n- Roles ({}): {role_list}", data.roles.len()));
                    }
                }
                field.value = guild_information;
            }
        }
    }

    if let Some(accent_colour) = user.accent_colour {
        embed.colour(accent_colour);
    } else if let Some(member) = &user.member {
        if let Some(data) = &member.data {
            if let Some(role) = data.highest_role_colour() {
                embed.colour(role.colour);
            }
        }
    }

    if let Some(ref banner) = user.banner_url() {
        embed.image(|i| i.url(banner));
    }

    embed.footer(|f| {
        f.text(format!("SYNCED | Information requested by {}", ctx.author.name()))
            .icon_url(ctx.author.avatar_url())
    });

    Ok(embed)
}

pub async fn info_button_guild_permissions<'a>(
    ctx: &ComponentInteraction,
    user: User,
    embed: &'a mut EmbedBuilder,
) -> anyhow::Result<&'a mut EmbedBuilder> {
    if let Some(ref member) = user.member {
        if let Some(ref data) = member.data {
            let permissions = data.role_permissions();
            let member_permissions = data.permission_calculator(&permissions);
            let mut present = false;

            for field in &embed.0.fields {
                if field.name.contains("Guild-Level Permissions") {
                    present = true;
                }
            }

            if !present {
                let mut permissions = vec![];
                for (permission, _) in member_permissions.root().iter_names() {
                    permissions.push(permission)
                }

                permissions.sort();

                embed
                    .create_field("Guild-Level Permissions", &format!("```\n{}```", permissions.join(" | ")), false)
                    .footer(|f| f.text(format!("Information requested by {}", ctx.author.name())));
            }

            return Ok(embed);
        }
    }

    ctx.respond(|r| r.content("Could not calculate permissions! Sorry!").ephemeral())
        .await?;

    Ok(embed)
}

pub async fn info_recent_messages(ctx: &ComponentInteraction, user: User) -> anyhow::Result<luro_model::types::CommandResponse> {
    let user_messages = ctx.database.sqlx.fetch_user_messages(user.user_id).await;

    ctx.respond(|r| {
        r.embed(|e| {
            for message in user_messages.values() {
                if !message.content.is_empty() {
                    let channel_id = message.channel_id;
                    let message_id = message.id;
                    e.create_field(
                        match message.guild_id {
                            Some(guild_id) => format!("https://discord.com/channels/{guild_id}/{channel_id}/{message_id}"),
                            None => format!("https://discord.com/channels/@me/{channel_id}/{message_id}"),
                        },
                        message.content.clone(),
                        false,
                    );
                }
            }

            e.colour(ctx.accent_colour())
                .author(|author| {
                    author
                        .name(format!("Recent Messages sent by {} | {}", user.name, user.user_id))
                        .icon_url(user.avatar_url())
                })
                .footer(|f| {
                    f.text(format!("Information requested by {}", ctx.author.name()))
                        .icon_url(ctx.author.avatar_url())
                })
        })
        .ephemeral()
    })
    .await
}

pub fn timestamps<'a>(author: &User, user: &User, embed: &'a mut EmbedBuilder) -> &'a mut EmbedBuilder {
    let mut present = false;

    for field in &embed.0.fields {
        if field.name.contains("Timestamps") {
            present = true;
        }
    }

    if !present {
        let mut timestamp = format!(
            "- Joined discord on <t:{0}> - <t:{0}:R>\n",
            Duration::from_millis(user.user_id.timestamp().unsigned_abs()).as_secs()
        );

        if let Some(member) = &user.member {
            timestamp.push_str(format!("- Joined this server at <t:{0}> - <t:{0}:R>\n", member.joined_at.as_secs()).as_str());

            if let Some(member_timestamp) = member.boosting_since {
                timestamp.push_str(format!("- Boosted this server since <t:{0}> - <t:{0}:R>\n", member_timestamp.as_secs()).as_str());
            }
            if let Some(ref data) = member.data {
                if let Some(left_at) = data.left_at {
                    timestamp.push_str(format!("- Left this server at <t:{0}> - <t:{0}:R>\n", left_at.unix_timestamp()).as_str());
                }
            }
        }

        embed.create_field("Timestamps", &timestamp, false).footer(|f| {
            f.text(format!("Information requested by {}", author.name()))
                .icon_url(author.avatar_url())
        });
    }

    embed
}

pub async fn luro_information<'a>(author: &User, user: &User, db: Arc<Database>, embed: &'a mut EmbedBuilder) -> &'a mut EmbedBuilder {
    let mut present = false;

    for field in &embed.0.fields {
        if field.name.contains("Luro Information") {
            present = true;
        }
    }

    if !present {
        let mut luro_information = String::new();

        if let Ok(user_characters) = db.user_fetch_characters(user.user_id).await {
            if !user_characters.is_empty() {
                luro_information.push_str(&format!("- Has `{}` character profiles\n", user_characters.len()));
            }
        }

        if let Ok(marriages) = db.user_fetch_marriages(user.user_id).await {
            let mut active_marriages = 0;

            for marriage in marriages {
                if !marriage.divorced && !marriage.rejected {
                    active_marriages += 1;
                }
            }

            if active_marriages != 0 {
                luro_information.push_str(&format!("- Has `{active_marriages}` marriages\n"));
            }
        }

        if let Some(ref user_data) = user.data {
            luro_information.push_str(&format!("- Is marked as `{}` in my database!\n", user_data.permissions));

            if let Some(gender) = &user_data.gender
                && let Some(sexuality) = &user_data.sexuality
            {
                luro_information.push_str(&format!("- Has a sexuality of `{sexuality}` and identifies as `{gender}`\n"));
            } else if let Some(gender) = &user_data.gender {
                luro_information.push_str(&format!("- Identifies as `{gender}`\n"));
            } else if let Some(sexuality) = &user_data.sexuality {
                luro_information.push_str(&format!("- Has a sexuality of `{sexuality}`\n"));
            }
        }

        if let Ok(word_count) = db.user_count_messages(user.user_id).await
            && word_count.total_messages.unwrap_or_default() != 0
        {
            if let Some(count) = word_count.total_messages
                && count != 0
            {
                luro_information.push_str(&format!("- Has sent `{count}` messages!\n"))
            };
            if let Some(count) = word_count.total_words
                && count != 0
            {
                luro_information.push_str(&format!("  - `{count}` words said!\n"))
            };
            if let Some(count) = word_count.total_unique_words
                && count != 0
            {
                luro_information.push_str(&format!("  - `{count}` unique words said!\n"))
            };
            if let Some(count) = word_count.total_custom_messages
                && count != 0
            {
                luro_information.push_str(&format!("  - `{count}` custom messages\n"))
            };
            if let Some(count) = word_count.total_message_creates
                && count != 0
            {
                luro_information.push_str(&format!("  - `{count}` messages created\n"))
            };
            if let Some(count) = word_count.total_message_cached
                && count != 0
            {
                luro_information.push_str(&format!("  - `{count}` messages cached\n"))
            };
            if let Some(count) = word_count.total_message_deletes
                && count != 0
            {
                luro_information.push_str(&format!("  - `{count}` messages deleted\n"))
            };
            if let Some(count) = word_count.total_message_updates
                && count != 0
            {
                luro_information.push_str(&format!("  - `{count}` messages updated\n"))
            };
            if let Some(count) = word_count.total_message_message
                && count != 0
            {
                luro_information.push_str(&format!("  - `{count}` messages stored\n"))
            };
        }

        if !luro_information.is_empty() {
            embed.create_field("Luro Information", &luro_information, false).footer(|f| {
                f.text(format!("Information requested by {}", author.name()))
                    .icon_url(author.avatar_url())
            });
        }
    }

    embed
}

pub fn user_information<'a>(author: &User, user: &User, embed: &'a mut EmbedBuilder) -> &'a mut EmbedBuilder {
    let mut present = false;

    for field in &embed.0.fields {
        if field.name.contains("User Information") {
            present = true;
        }
    }

    if let Some(ref banner) = user.banner_url() {
        embed.image(|i| i.url(banner));
    }

    if !present {
        let mut user_information = String::new();
        if let Some(flags) = &user.flags
            && !flags.is_empty()
        {
            if flags.bits() == 1 << 20 {
                user_information.push_str("**USER IS MARKED FOR UNUSUAL AMOUNTS OF DMS**")
            }

            let mut flags_sorted = vec![];
            for (flag, _) in flags.iter_names() {
                flags_sorted.push(flag)
            }
            flags_sorted.sort();
            user_information.push_str(&format!(
                "\n- User Flags (`{}`): \n```\n{}```",
                flags.bits(),
                flags_sorted.join(" | ")
            ));
        }

        if let Some(flags) = &user.public_flags
            && !flags.is_empty()
        {
            if flags.bits() == 1 << 20 {
                user_information.push_str("**USER IS MARKED FOR UNUSUAL AMOUNTS OF DMS**")
            }

            let mut flags_sorted = vec![];
            for (flag, _) in flags.iter_names() {
                flags_sorted.push(flag)
            }
            flags_sorted.sort();
            user_information.push_str(&format!(
                "\n- Public Flags (`{}`): \n```\n{}```",
                flags.bits(),
                flags_sorted.join(" | ")
            ))
        }

        if let Some(accent_color) = user.accent_colour {
            embed.colour(accent_color);
            user_information.push_str(&format!("\n- Accent Colour: `{accent_color:X}`"));
        } else if let Some(member) = &user.member {
            if let Some(data) = &member.data {
                if let Some(role) = data.highest_role_colour() {
                    embed.colour(role.colour);
                    user_information.push_str(&format!("\n- Accent Colour: `{:X}` (<@&{}>)", role.colour, role));
                }
            }
        }

        if let Some(email) = &user.email {
            user_information.push_str(&format!("\n- Email: `{}`", email));
        }
        if let Some(locale) = &user.locale {
            user_information.push_str(&format!("\n- Locale: `{}`", locale));
        }
        if user.mfa_enabled.unwrap_or_default() {
            user_information.push_str("\n- MFA Enabled: `true`");
        }
        if user.system.unwrap_or_default() {
            user_information.push_str("\n- System Account: `true`");
        }
        if user.verified.unwrap_or_default() {
            user_information.push_str("\n- Verified Account: `true`");
        }
        if user.bot {
            user_information.push_str("\n- Bot: `true`");
        }

        embed.footer(|f| {
            f.text(format!("Information requested by {}", author.name()))
                .icon_url(author.avatar_url())
        });
        match user_information.is_empty() {
            true => embed.create_field("User Information", "- I don't have any user information to report!", false),
            false => embed.create_field("User Information", &user_information, false),
        };
    }

    embed
}

pub fn guild_information<'a>(author: &User, member: &Member, embed: &'a mut EmbedBuilder) -> &'a mut EmbedBuilder {
    let mut present = false;

    for field in &embed.0.fields {
        if field.name.contains("Guild Information") {
            present = true;
        }
    }

    if !present {
        let mut guild_information = String::new();
        let mut role_list = String::new();

        if let Some(nickname) = &member.nickname {
            guild_information.push_str(&format!("\n- Nickname: `{nickname}`"));
        }
        if member.deafened {
            guild_information.push_str("\n- Deafened: `true`");
        }
        if member.muted {
            guild_information.push_str("\n- Muted: `true`");
        }
        if member.pending {
            guild_information.push_str("\n- Pending: `true`");
        }

        let mut flags_sorted = vec![];

        for (flag, _) in member.flags.iter_names() {
            flags_sorted.push(flag)
        }
        flags_sorted.sort();
        if !flags_sorted.is_empty() {
            guild_information.push_str(&format!(
                "\n- Member Flags ({}): \n```\n{}```",
                member.flags.bits(),
                flags_sorted.join(" | ")
            ));
        }

        // TODO: Once member_banner is a thing in [Member]
        // if let Some(banner) = get_member_banner(&member, guild_id, user) {
        //     embed = embed.image(ImageSource::url(banner)?)
        // }

        if let Some(ref data) = member.data {
            if data.guild_owner {
                guild_information.push_str("\n- Is the owner of this guild!");
            }

            for role_id in data.sorted_roles() {
                if role_list.is_empty() {
                    role_list.push_str(&format!("<@&{role_id}>"));
                    continue;
                };
                role_list.push_str(&format!(", <@&{role_id}>"));
            }

            if !role_list.is_empty() {
                guild_information.push_str(&format!("\n- Roles ({}): {role_list}", data.roles.len()));
            }
        }

        if !guild_information.is_empty() {
            embed.create_field("Guild Information", &guild_information, false).footer(|f| {
                f.text(format!("Information requested by {}", author.name()))
                    .icon_url(author.avatar_url())
            });
        }
    }

    embed
}

pub fn buttons(guild_id: Option<Id<GuildMarker>>, show_buttons: bool) -> ComponentBuilder {
    let mut components = ComponentBuilder::default();
    if show_buttons {
        components.action_row(|a_r| {
            a_r.button(|b| {
                b.custom_id("info-button-messages")
                    .label("Messages by user")
                    .style(ButtonStyle::Secondary)
            })
            .button(|b| {
                b.custom_id("info-button-timestamps")
                    .label("User Timestamps")
                    .style(ButtonStyle::Secondary)
            })
            .button(|b| {
                b.custom_id("info-button-user")
                    .label("User Information")
                    .style(ButtonStyle::Secondary)
            })
            .button(|b| {
                b.custom_id("info-button-luro")
                    .label("Luro Information")
                    .style(ButtonStyle::Secondary)
            })
        });
    }
    components.action_row(|a_r| {
        if guild_id.is_some() && show_buttons {
            a_r.button(|b| {
                b.custom_id("info-button-guild")
                    .label("Member Information")
                    .style(ButtonStyle::Secondary)
            })
            .button(|b| {
                b.custom_id("info-button-guild-permissions")
                    .label("Member Permissions")
                    .style(ButtonStyle::Secondary)
            });
        }
        a_r.button(|b| b.custom_id("info-button-clear").label("Clear Embed").style(ButtonStyle::Danger))
            .button(|b| b.custom_id("info-button-sync").label("Sync").style(ButtonStyle::Primary))
    });
    components
}
