use luro_utilities::guild_accent_colour;

use luro_core::{Context, Error};

/// Print information of the guilds I'm in!
#[poise::command(slash_command, prefix_command, category = "Owner")]
pub async fn guilds(ctx: Context<'_>) -> Result<(), Error> {
    use std::fmt::Write as _;

    let show_private_guilds = ctx.framework().options().owners.contains(&ctx.author().id);
    let accent_colour = ctx.data().config.read().await.accent_colour;

    /// Stores details of a guild for the purposes of listing it in the bot guild list
    struct Guild {
        /// Name of the guild
        name: String,
        /// Number of members in the guild
        num_members: u64,
        /// Whether the guild is public
        is_public: bool,
        /// The guild ID
        guild_id: u64
    }

    let guild_ids = ctx.sc().cache.guilds();
    let mut num_unavailable_guilds = 0;
    let mut guilds = guild_ids
        .iter()
        .map(|&guild_id| {
            ctx.sc().cache.guild_field(guild_id, |guild| Guild {
                name: guild.name.clone(),
                num_members: guild.member_count,
                is_public: guild.features.iter().any(|x| x == "DISCOVERABLE"),
                guild_id: guild.id.0
            })
        })
        .filter_map(|guild| {
            if guild.is_none() {
                num_unavailable_guilds += 1;
            }
            guild
        })
        .collect::<Vec<_>>();
    guilds.sort_by_key(|guild| u64::MAX - guild.num_members); // descending sort

    let mut num_private_guilds = 0;
    let mut num_private_guild_members = 0;
    let mut response = format!("I am currently in {} servers!\n", guild_ids.len());
    for guild in guilds {
        if guild.is_public || show_private_guilds {
            let _ = writeln!(
                response,
                "- **{}** ({} members) `{}`",
                guild.name, guild.num_members, guild.guild_id
            );
        } else {
            num_private_guilds += 1;
            num_private_guild_members += guild.num_members;
        }
    }
    if num_private_guilds > 0 {
        let _ = writeln!(
            response,
            "- [{num_private_guilds} private servers with {num_private_guild_members} members total]"
        );
    }
    if num_unavailable_guilds > 0 {
        let _ = writeln!(
            response,
            "- [{num_unavailable_guilds} unavailable servers (cache is not ready yet)]"
        );
    }

    if show_private_guilds {
        response += "\n_Showing private guilds because you are the bot owner_\n";
    }

    let bot_user = ctx.framework().bot_id.to_user(ctx).await;

    ctx.send(|b| {
        b.embed(|embed| {
            embed
                .description(response)
                .color(guild_accent_colour(accent_colour, ctx.guild()));

            if let Ok(bot_user) = bot_user {
                embed.author(|author| {
                    author
                        .name(&bot_user.name)
                        .icon_url(&bot_user.avatar_url().unwrap_or_default())
                });
            }
            embed
        })
        .ephemeral(show_private_guilds)
    })
    .await?;

    Ok(())
}
