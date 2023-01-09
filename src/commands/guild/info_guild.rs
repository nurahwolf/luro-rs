use crate::{
    functions::{guild_accent_colour::guild_accent_colour, sort_roles::sort_roles},
    Context, Error
};
use itertools::Itertools;
use poise::serenity_prelude::{ChannelType, CreateEmbed, Guild, NsfwLevel, CacheHttp, invite};
use std::fmt::Write;

/// Information about the guild you are in
#[poise::command(prefix_command, slash_command, guild_only, category = "Guild")]
pub async fn guild(ctx: Context<'_>, #[description = "The guild to look up"] guild: Option<Guild>) -> Result<(), Error> {
    // Return the current guild, or the guild the user requested
    let guild_resolved = match &guild {
        Some(guild_specified) => ctx.serenity_context().cache.guild(guild_specified).ok_or("Could not find the guild you entered")?,
        None => ctx.guild().ok_or("Could not find the guild I am in")?
    };

    // Variables
    let mut roles_sorted = sort_roles(&guild_resolved);
    let highest_role = roles_sorted.next().expect("No roles in the server, somehow");

    // Create an embed for the data we wish to show, filling it with key data
    let mut embed = CreateEmbed::default();
    embed.colour(guild_accent_colour(ctx.data().config.read().await.accent_colour, Some(guild_resolved.to_owned())));
    embed.title(&guild_resolved.name);
    embed.thumbnail(&guild_resolved.icon_url().unwrap_or_default());

    // Information that does not need its own field
    let mut description = String::new();
    let guild_channels: Vec<_> = guild_resolved.channels.iter().filter_map(|(_, c)| c.clone().guild()).collect();
    let guild_channels_all = guild_channels.len();
    let guild_channels_text = guild_channels.iter().filter(|c| c.kind == ChannelType::Text).count();
    let guild_channels_voice = guild_channels.iter().filter(|c| c.kind == ChannelType::Voice).count();

    writeln!(description, "Guild Created: <t:{0}>", ctx.guild_id().expect("Guild not found").created_at().unix_timestamp())?;
    if guild.is_none() {
        writeln!(description, "Highest Role: {}", highest_role.1.name)?;
    } else {
        writeln!(description, "Highest Role: {}", highest_role.1)?;
    };
    writeln!(description, "Online Members: {}", &guild_resolved.presences.len())?;
    writeln!(description, "Total Members: {}", &guild_resolved.members.len())?;
    writeln!(description, "Total Channels: {guild_channels_all} ({guild_channels_text} text, {guild_channels_voice} voice)")?;

    embed.description(description);

    // Display guild banner if it exists
    if let Some(banner) = &guild_resolved.banner_url() {
        embed.image(banner);
    };

    // Display the guild owner as the embed author
    if let Ok(guild_owner) = &guild_resolved.member(ctx, &guild_resolved.owner_id).await {
        embed.author(|author| {
            author
                .icon_url(guild_owner.avatar_url().unwrap_or_default())
                .name(format!("Server Owner: {0}", guild_owner.user.tag()))
        });
    };

    if let Ok(invites) = guild_resolved.invites(ctx).await {
        if let Some(invite) = invites.first() {
            embed.field("Invite", invite.url(), false);
        }
    }

    // Nitro Boost Information
    if guild_resolved.premium_subscription_count != 0 {
        let guild_boost_tier = match guild_resolved.premium_tier.num() {
            0 => "No current tier (not boosted)",
            1 => "Level 1 (2+ boosts)",
            2 => "Level 2 (7+ boosts)",
            3 => "Level 3 (14+ boosts)",
            _ => "Unrecognized boost tier."
        };
        embed.field(
            "Nitro Statistics",
            format!("**Total Boosts:** {0}\n**Boost Tier:** {guild_boost_tier}", guild_resolved.premium_subscription_count),
            false
        );
    };

    // NSFW level
    if guild_resolved.nsfw_level != NsfwLevel::Default {
        let nsfw_level = match guild_resolved.nsfw_level {
            NsfwLevel::Default => "The guild does not have a NSFW level",
            NsfwLevel::Explicit => "The guild is considered to be explicit.",
            NsfwLevel::Safe => "The guild is considered to be safe.",
            NsfwLevel::AgeRestricted => "The guild is age restricted.",
            _ => "Unknown nsfw level."
        };
        embed.field("NSFW Guild", nsfw_level, false);
    };

    // Verification Level
    embed.field(
        "Verification Level",
        match guild_resolved.verification_level.num() {
            0 => "None - Unrestricted.",
            1 => "Low - Must have a verified email.",
            2 => "Medium - Registered on Discord for 5+ minutes.",
            3 => "(╯°□°)╯︵ ┻━┻ - In the server for 10+ minutes.",
            4 => "┻━┻ ﾐヽ(ಠ益ಠ)/彡┻━┻) - Must have a verified phone number.",
            _ => "Unrecognized verification level."
        },
        false
    );

    // MFA Level
    embed.field(
        "MFA Level",
        match guild_resolved.mfa_level.num() {
            0 => "Multi-factor authentication not required.",
            1 => "Multi-factor authentication required.",
            _ => "Unrecognized multi-factor authentication level."
        },
        false
    );

    // Explicit Filter
    embed.field(
        "Explicit Filter",
        match guild_resolved.explicit_content_filter.num() {
            0 => "Disabled".to_owned(),
            1 => "Media scanned from members w/o a role.".to_owned(),
            2 => "Everyone".to_owned(),
            _ => "Unrecognized filter setting.".to_owned()
        },
        false
    );

    // System Channels
    let mut system_channels = String::new();
    if let Some(system_channel) = &guild_resolved.system_channel_id {
        if let Some(system_channel) = guild_resolved.channels.get(system_channel) {
            writeln!(system_channels, "System Channel: {system_channel}")?;
        };
    };

    if let Some(default_channel) = guild_resolved.default_channel(ctx.author().id).await {
        writeln!(system_channels, "Default Channel: {default_channel}")?;
    };

    if let Some(afk_channel) = &guild_resolved.afk_channel_id {
        if let Some(afk_channel) = guild_resolved.channels.get(afk_channel) {
            writeln!(system_channels, "AFK Channel: {afk_channel}")?;
        };
    };

    if let Some(rules_channel) = &guild_resolved.rules_channel_id {
        if let Some(rules_channel) = guild_resolved.channels.get(rules_channel) {
            writeln!(system_channels, "Rules Channel: {rules_channel}")?;
        };
    };

    if let Some(widget_channel) = &guild_resolved.widget_channel_id {
        if let Some(widget_channel) = guild_resolved.channels.get(widget_channel) {
            writeln!(system_channels, "Widget Channel: {widget_channel}")?;
        };
    };

    if let Some(public_update_channel) = &guild_resolved.public_updates_channel_id {
        if let Some(public_update_channel) = guild_resolved.channels.get(public_update_channel) {
            writeln!(system_channels, "Public Update Channel: {public_update_channel}")?;
        };
    };

    embed.field("System Channels", system_channels, false);

    // Server Roles
    let mut all_roles_string = String::new();
    for (_, val) in roles_sorted.clone() {
        if all_roles_string.len() <= 1000 {
            // Stop adding roles once we hit 1000 characters
            if guild.is_none() {
                write!(all_roles_string, "{val}, ")?;
            } else {
                write!(all_roles_string, "{0}, ", val.name)?;
            }
        }
    }
    all_roles_string.truncate(1024); // Additional catch, just to make sure we are within the limit!
    embed.field("Server Roles", all_roles_string, false);

    // Guild Features
    if !&guild_resolved.features.is_empty() {
        embed.field("Guild Features", guild_resolved.features.iter().join(", "), false);
    };

    // Emoji Stats
    if !&guild_resolved.emojis.is_empty() {
        let guild_emojis = &guild_resolved.emojis.len();
        let guild_emojis_animated = &guild_resolved.emojis.iter().filter(|(_, e)| e.animated).count();
        let guild_emojis_normal = &guild_resolved.emojis.iter().filter(|(_, e)| !e.animated).count();
        embed.field(
            "Emoji Statistics",
            format!("**Animated Emojis:** {guild_emojis_animated}\n**Normal Emojis:** {guild_emojis_normal}\n**Total Emojis:** {guild_emojis}"),
            false
        );
    };

    // Send the embed
    ctx.send(|builder| {
        builder.embed(|f| {
            *f = embed;
            f
        })
    })
    .await?;

    Ok(())
}
