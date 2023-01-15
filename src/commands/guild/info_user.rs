use itertools::Itertools;
use poise::serenity_prelude::{ActivityType, CacheHttp, CreateEmbed, Guild, OnlineStatus, User, UserPublicFlags};
use std::fmt::Write;

use crate::{
    functions::{accent_colour::accent_colour, guild_accent_colour::guild_accent_colour},
    Context, Error
};

async fn user_info(ctx: Context<'_>, user: User, guild: Option<Guild>) -> Result<(), Error> {
    // Variables
    let mut activity_string = String::new();
    let mut track_art = String::new();

    // Setup our embed
    let mut embed = CreateEmbed::default();

    // If guild is specified / we are in a guild, print some extra information
    if let Some(guild) = match guild {
        Some(guild_specified) => ctx.serenity_context().cache.guild(guild_specified),
        None => ctx.guild()
    } {
        // Now that we have a guild, check we can resolve the user. Panic if we can't.
        let guild_user = match guild.member(ctx, &user).await {
            Ok(member_resolved) => member_resolved,
            Err(error) => {
                ctx.say(format!("Failed to resolve guild user because: {error}")).await?;
                return Ok(());
            }
        };

        if let Some(presence) = guild.presences.get(&guild_user.user.id) {
            for activity in &presence.activities {
                match activity.kind {
                    ActivityType::Listening => {
                        if activity.name == "Spotify" {
                            writeln!(activity_string, "Using Spotify")?;

                            if let Some(song) = &activity.details {
                                writeln!(activity_string, "- playing {song}")?;
                            }

                            if let Some(uri) = &activity.sync_id {
                                writeln!(activity_string, "- song link: https://open.spotify.com/track/{uri}")?;
                            }

                            if let Some(assets) = &activity.assets {
                                if let Some(album) = &assets.large_text {
                                    if let Some(artwork) = &assets.large_text {
                                        let artwork = artwork.replace("spotify:", "");
                                        let artwork_url = format!("https://i.scdn.co/image/{artwork}");
                                        track_art.push_str(&artwork_url);

                                        if let Some(artists) = &activity.state {
                                            let mut artist_string = artists.to_string();
                                            if artists.contains(';') {
                                                let replacer = artist_string.replace(';', ",");
                                                let commas = replacer.matches(", ").count();
                                                let rfind = artist_string.rfind(';').unwrap();
                                                let (left, right) = replacer.split_at(rfind);
                                                let format_string = if commas >= 2 {
                                                    format!("{left}{}", right.replace(',', ", &"))
                                                } else {
                                                    format!("{left} {}", right.replace(',', "&"))
                                                };

                                                artist_string.clear();
                                                artist_string.push_str(&format_string);
                                                writeln!(activity_string, "- {album} by {artist_string}")?;
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            writeln!(activity_string, "Listening to music")?;
                        }
                    }
                    ActivityType::Playing => {
                        if activity.name == "Visual Studio Code" {
                            writeln!(activity_string, "Using Visual Studio Code")?;
                            if let Some(task) = &activity.details {
                                writeln!(activity_string, "- {task}")?;
                            }

                            if let Some(project) = &activity.state {
                                writeln!(activity_string, "- {project}")?;
                            }
                        } else {
                            writeln!(activity_string, "Playing")?
                        }
                    }
                    ActivityType::Competing => writeln!(activity_string, "Competing in {}", activity.name)?,
                    ActivityType::Streaming => writeln!(activity_string, "Streaming in {}", activity.name)?,
                    _ => {}
                };
            }
            if !activity_string.is_empty() {
                embed.field("Activities", activity_string, false);
            }

            embed.field(
                "Online",
                match presence.status {
                    OnlineStatus::Online => "Online",
                    OnlineStatus::Idle => "Idle",
                    OnlineStatus::DoNotDisturb => "Do Not Disturb",
                    OnlineStatus::Invisible => "Invisible",
                    _ => "Offline"
                },
                false
            );

            if let Some(client_status) = &presence.client_status {
                embed.field(
                    "Device",
                    {
                        if client_status.desktop.is_some() && client_status.mobile.is_none() && client_status.web.is_none() {
                            "Desktop"
                        } else if client_status.mobile.is_some()
                            && client_status.desktop.is_none()
                            && client_status.web.is_none()
                        {
                            "Mobile"
                        } else if client_status.web.is_some()
                            && client_status.desktop.is_none()
                            && client_status.mobile.is_none()
                        {
                            "Web"
                        } else if client_status.desktop.is_some()
                            && client_status.mobile.is_some()
                            && client_status.web.is_none()
                        {
                            "Desktop and Mobile"
                        } else if client_status.desktop.is_some()
                            && client_status.mobile.is_some()
                            && client_status.web.is_some()
                        {
                            "Desktop, Mobile, and Web"
                        } else if client_status.mobile.is_some()
                            && client_status.web.is_some()
                            && client_status.desktop.is_none()
                        {
                            "Mobile and Web"
                        } else {
                            "Desktop and Web"
                        }
                    },
                    false
                );
            }
        }

        // User timestamps
        if let Some(joined_at) = guild_user.joined_at {
            embed.field(
                "Timestamps",
                format!(
                    "Joined discord on <t:{0}>\nJoined this server at <t:{1}>",
                    guild_user.user.created_at().unix_timestamp(),
                    joined_at.unix_timestamp()
                ),
                false
            );
        }

        if let Some(permissions) = guild_user.permissions {
            embed.field("Permissions", permissions, false);
        }

        if let Some(user_roles) = guild_user.roles(ctx) {
            // Server Roles
            let mut user_roles_string = String::new();
            let same_guild = if let Some(ctx_guild) = ctx.guild() {
                guild.id == ctx_guild.id
            } else {
                false
            };
            let sorted_roles = user_roles.iter().sorted_by_key(|r| r.position).rev();

            for role in sorted_roles {
                if user_roles_string.len() <= 1000 {
                    // Stop adding roles once we hit 1000 characters
                    if same_guild {
                        write!(user_roles_string, "{role}, ")?;
                    } else {
                        write!(user_roles_string, "{}, ", role.name)?;
                    }
                }
            }
            user_roles_string.truncate(1024); // Additional catch, just to make sure we are within the limit!
            if !user_roles_string.is_empty() {
                embed.field("User Roles", user_roles_string, false);
            }

            if let Some(roleid) = guild_user.highest_role_info(ctx) {
                if let Some(role) = guild.roles.get(&roleid.0) {
                    if same_guild {
                        embed.field("Highest Role", role, false);
                    } else {
                        embed.field("Highest Role", &role.name, false);
                    }
                }
            }
        }

        // If they have a nickname set
        if let Some(nickname) = &guild_user.nick {
            embed.field("Nickname", nickname, false);
        }

        if !track_art.is_empty() {
            embed.thumbnail(track_art);
        } else {
            embed.thumbnail(
                &guild_user
                    .avatar_url()
                    .unwrap_or(guild_user.user.avatar_url().unwrap_or_default())
            );
        }

        // Display the guild owner as the embed author
        embed.author(|author| author.icon_url(&guild.icon_url().unwrap_or_default()).name(&guild.name));

        if let Some(colour) = guild_user.colour(ctx) {
            embed.field("User's colour", format!("#{}", colour.hex()), false);
            embed.colour(colour);
        } else {
            embed.color(guild_accent_colour(ctx.data().config.read().await.accent_colour, Some(guild)));
        };
    } else {
        // We are not in a guild or a guild has not been specified, so print some basic information.

        // Set accent colour
        embed.color(accent_colour(ctx.data().config.read().await.accent_colour));

        // Set avatar as thumbnail
        if !track_art.is_empty() {
            embed.thumbnail(track_art);
        } else {
            embed.thumbnail(user.avatar_url().unwrap_or_default());
        }

        // Display the guild owner as the embed author
        embed.author(|author| author.icon_url(&user.avatar_url().unwrap_or_default()).name(&user.name));
    }

    // Information on the user that stays the same regardless of if they are in a guild
    embed.field("Account Type", if user.bot { "Bot" } else { "User" }, false);
    embed.description(format!("**User:** {} (ID: {})", &user, user.id.0));

    // Apparently can only get some information over the rest API, awesome...
    if let Ok(user_rest) = ctx.http().get_user(user.id.0).await {
        if let Some(banner_url) = user_rest.banner_url() {
            embed.image(banner_url);
        }

        if let Some(accent_colour) = user_rest.accent_colour {
            embed.field("Banner Accent Colour", accent_colour.hex(), false);
        }

        if let Some(flags) = user_rest.public_flags {
            embed.field(
                "User Flags",
                match flags {
                    UserPublicFlags::BOT_HTTP_INTERACTIONS => "BOT_HTTP_INTERACTIONS",
                    UserPublicFlags::BUG_HUNTER_LEVEL_1 => "BUG_HUNTER_LEVEL_1",
                    UserPublicFlags::BUG_HUNTER_LEVEL_2 => "BUG_HUNTER_LEVEL_2",
                    UserPublicFlags::DISCORD_CERTIFIED_MODERATOR => "DISCORD_CERTIFIED_MODERATOR",
                    UserPublicFlags::DISCORD_EMPLOYEE => "DISCORD_EMPLOYEE",
                    UserPublicFlags::EARLY_SUPPORTER => "EARLY_SUPPORTER",
                    UserPublicFlags::EARLY_VERIFIED_BOT_DEVELOPER => "EARLY_VERIFIED_BOT_DEVELOPER",
                    UserPublicFlags::HOUSE_BALANCE => "HOUSE_BALANCE",
                    UserPublicFlags::HOUSE_BRAVERY => "HOUSE_BRAVERY",
                    UserPublicFlags::HOUSE_BRILLIANCE => "HOUSE_BRILLIANCE",
                    UserPublicFlags::HYPESQUAD_EVENTS => "HYPESQUAD_EVENTS",
                    UserPublicFlags::PARTNERED_SERVER_OWNER => "PARTNERED_SERVER_OWNER",
                    UserPublicFlags::SYSTEM => "SYSTEM",
                    UserPublicFlags::TEAM_USER => "TEAM_USER",
                    UserPublicFlags::VERIFIED_BOT => "VERIFIED_BOT",
                    _ => "Unknown"
                },
                false
            );
        }
    }

    // Send the embed
    match ctx
        .send(|builder| {
            builder.embed(|f| {
                *f = embed;
                f
            })
        })
        .await
    {
        Ok(_) => {}
        Err(error) => match error {
            serenity::Error::Http(err) => {
                ctx.say(format!("**The embed fucked up:**\n{err}")).await?;
            }
            _ => {
                ctx.say(format!("Had a fuckywucky: {error}")).await?;
            }
        }
    };

    Ok(())
}

/// Show some information about a user
#[poise::command(prefix_command, slash_command, category = "Guild")]
pub async fn user(
    ctx: Context<'_>,
    #[description = "The user to get"] user: Option<User>,
    #[description = "Specify a guild if you wish to get their information from that guild"] guild: Option<Guild>
) -> Result<(), Error> {
    // Get the user, otherwise set the message author as the user to get
    let user = match user {
        Some(user_specified) => user_specified,
        None => ctx.author().clone()
    };

    user_info(ctx, user, guild).await?;
    Ok(())
}

/// Show some information about a user
#[poise::command(category = "Guild", context_menu_command = "User info")]
pub async fn userinfo_context(ctx: Context<'_>, #[description = "The user to get"] user: User) -> Result<(), Error> {
    user_info(ctx, user, None).await?;
    Ok(())
}
