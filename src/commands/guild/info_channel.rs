use poise::serenity_prelude::{Channel, ChannelType, CreateEmbed, Guild};
use std::fmt::Write;

use crate::{
    functions::{accent_colour::accent_colour, guild_accent_colour::guild_accent_colour},
    Context, Error
};

/// Get information on a channel in a guild
#[poise::command(slash_command, prefix_command, guild_only, category = "Guild")]
pub async fn channel(
    ctx: Context<'_>,
    #[description = "The channel to look up"] channel: Option<Channel>,
    #[description = "The guild to look up, if the channel is from a different server"] guild: Option<Guild>
) -> Result<(), Error> {
    // Returns a channel
    let regular_channel = match channel {
        Some(channel_resolved) => channel_resolved,
        None => ctx
            .serenity_context()
            .cache
            .channel(ctx.channel_id())
            .ok_or("Could not find the channel (Did you specify the server if running in DMs?")?
    };

    // Create an embed that we will send and start filling it with data
    let mut embed = CreateEmbed::default();
    embed.colour(accent_colour(ctx.data().config.lock().unwrap().accent_colour));
    embed.title(&regular_channel);

    // If we can resolve this channel as a guild, add some extra stuff
    if let Some(guild_channel) = ctx.serenity_context().cache.guild_channel(regular_channel.id()) {
        // Return the current guild, or the guild the user requested
        let guild_resolved = match &guild {
            Some(guild_specified) => ctx.serenity_context().cache.guild(guild_specified).ok_or("Could not find the guild you entered")?,
            None => ctx.guild().ok_or("Could not find the guild I am in")?
        };

        // More embed info with some overrides
        embed.title(&guild_channel.name);
        embed.colour(guild_accent_colour(ctx.data().config.lock().unwrap().accent_colour, Some(guild_resolved.to_owned())));
        embed.thumbnail(&guild_resolved.icon_url().unwrap_or_default());

        if let Some(topic) = guild_channel.topic {
            embed.description(topic);
        }
        embed.field("Channel Position", guild_channel.position, false);

        if let Some(parent_id) = guild_channel.parent_id {
            if let Some(parent_channel) = guild_resolved.channels.get(&parent_id) {
                embed.field("Channel Parent", parent_channel, false);
            }
        }

        // Display the guild owner as the embed author
        if let Ok(guild_owner) = &guild_resolved.member(ctx, &guild_resolved.owner_id).await {
            embed.author(|author| {
                author
                    .icon_url(guild_owner.avatar_url().unwrap_or_default())
                    .name(format!("Server Owner: {0}", guild_owner.user.tag()))
            });
        };

        // Display guild banner if it exists
        if let Some(banner) = &guild_resolved.banner_url() {
            embed.image(banner);
        };

        if guild_channel.nsfw {
            embed.field("NSFW Channel", "Yes it is you filthy pervert 🔞", false);
        }

        // Print the channel type
        let channel_type = match guild_channel.kind {
            ChannelType::Text => "A regular text channel",
            ChannelType::Private => "A private channel",
            ChannelType::Voice => "A voice channel",
            ChannelType::Category => "A category channel",
            ChannelType::News => "A news channel",
            ChannelType::NewsThread => "A news thread type channel",
            ChannelType::PublicThread => "A public thread channel",
            ChannelType::PrivateThread => "A public thread channel",
            ChannelType::Stage => "A stage channel",
            ChannelType::Directory => "A directory channel",
            ChannelType::Forum => "A directory channel",
            _ => "Unknown channel type"
        };
        embed.field("Channel Type", channel_type, false);

        if let Some(channel_bitrate) = guild_channel.bitrate {
            embed.field("Channel Bitrate", channel_bitrate, false);
        }

        if let Some(default_archive_duration) = guild_channel.default_auto_archive_duration {
            embed.field("Auto Archive duration", default_archive_duration, false);
        }

        if let Some(last_pin_timestamp) = guild_channel.last_pin_timestamp {
            embed.field("Last Message Pinned", format!("<t:{}>", last_pin_timestamp.unix_timestamp()), false);
        }

        if let Some(member_count) = guild_channel.member_count {
            embed.field("Number of members in thread", member_count, false);
        }

        if let Some(user_limit) = guild_channel.user_limit {
            embed.field("User limit of this channel", user_limit, false);
        }

        if let Some(rate_limit) = guild_channel.rate_limit_per_user {
            if rate_limit != 0 {
                embed.field("Channel rate limit in seconds", rate_limit, false);
            }
        }

        if let Some(rtc_region) = guild_channel.rtc_region {
            embed.field("RTC Region", rtc_region, false);
        }
    };

    // Channel Category
    if let Some(category) = regular_channel.category() {
        let mut channel_category_info = String::new();
        let category_type = match category.kind {
            ChannelType::Text => "A regular text channel",
            ChannelType::Private => "A private channel",
            ChannelType::Voice => "A voice channel",
            ChannelType::Category => "A category channel",
            ChannelType::News => "A news channel",
            ChannelType::NewsThread => "A news thread type channel",
            ChannelType::PublicThread => "A public thread channel",
            ChannelType::PrivateThread => "A public thread channel",
            ChannelType::Stage => "A stage channel",
            ChannelType::Directory => "A directory channel",
            ChannelType::Forum => "A directory channel",
            _ => "Unknown channel type"
        };
        if let Some(parent_category_id) = category.parent_id {
            writeln!(channel_category_info, "Parent Category: <@{parent_category_id}>")?;
        }
        writeln!(channel_category_info, "Category: {}", category.name())?;
        writeln!(channel_category_info, "Category position: {}", category.position)?;
        writeln!(channel_category_info, "Category type: {category_type}")?;
        writeln!(channel_category_info, "Belongs to guild: {}", category.guild_id.name(ctx).unwrap())?;
        writeln!(channel_category_info, "NSFW Category: {}", category.nsfw)?;
        embed.field("Category Information", channel_category_info, false);
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
