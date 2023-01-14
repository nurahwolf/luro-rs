use crate::{functions::guild_accent_colour::guild_accent_colour, Context, Error};
use poise::serenity_prelude::{CreateEmbed, Role};
use std::fmt::Write;

/// Information about a role
#[poise::command(prefix_command, slash_command, guild_only, category = "Guild")]
pub async fn role(
    ctx: Context<'_>,
    #[description = "The role you want to inspect"] role: Role,
    #[description = "Plaintext usernames instead of mentionable usernames"]
    #[flag]
    plaintext_users: bool
) -> Result<(), Error> {
    // Get the guild the user is in.
    let guild = ctx.guild().ok_or("Could not find the guild I am in")?;
    let accent_colour = ctx.data().config.read().await.accent_colour;

    // Create an embed for the data we wish to show, filling it with key data
    let mut embed = CreateEmbed::default();
    embed.colour(guild_accent_colour(accent_colour, ctx.guild()));
    embed.title(&role.name);
    embed.thumbnail(&guild.icon_url().unwrap_or_default());
    if let Some(guild_banner) = &guild.banner_url() {
        embed.image(guild_banner);
    }
    embed.field("Permissions", role.permissions, false);

    let mut user_list = String::new();
    let members_with_role: Vec<_> = guild.members.iter().filter(|filter| filter.1.roles.contains(&role.id)).collect();
    for member in &members_with_role {
        if user_list.len() <= 4000 {
            if plaintext_users {
                write!(user_list, "{}, ", member.1.display_name())?;
            } else {
                write!(user_list, "{}, ", member.1)?;
            }
        }
    }
    embed.field("Total users with role", members_with_role.len(), true);
    user_list.truncate(4000);
    embed.description(format!("**Users with role:**\n{user_list}"));

    if let Some(unicode_emoji) = &role.unicode_emoji {
        embed.field("Unicode Emoji", unicode_emoji, true);
    }

    if role.colour.0 != 0 {
        embed.field("Colour", role.colour.hex(), true);
    }
    embed.field("Role ID", role.id.0, true);
    embed.field("Position", role.position, true);

    if role.hoist {
        embed.field("Hoised", "True", true);
    }

    if role.managed {
        embed.field("Managed", "True", true);
    }

    if role.mentionable {
        embed.field("Mentionable", "True", true);
    }

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
