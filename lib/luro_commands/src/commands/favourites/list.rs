use luro_core::{Context, Error};
use luro_utilities::guild_accent_colour;
use poise::serenity_prelude::CreateEmbed;

/// List your favorites
#[poise::command(slash_command, category = "Favourites")]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    // Get favourites and accent_colour from datastore / config
    let favourites = &ctx.data().user_favourites.read().await.favs;
    let accent_colour = ctx.data().config.read().await.accent_colour;
    let mut embed = CreateEmbed::default();
    embed.title("Your Favourites");
    embed.color(guild_accent_colour(accent_colour, ctx.guild()));
    match ctx.author_member().await {
        Some(author_member) => embed.author(|author| {
            author
                .name(author_member.display_name())
                .icon_url(author_member.avatar_url().unwrap_or_default())
        }),
        None => embed.author(|author| {
            author
                .name(ctx.author().name.clone())
                .icon_url(ctx.author().avatar_url().unwrap_or_default())
        })
    };

    // Get favorites from author
    let user_favourites = match favourites.get(&ctx.author().id.to_string()) {
        Some(ok) => ok,
        None => {
            ctx.say("Looks like you don't have any favorites saved yet!").await?;
            return Ok(());
        }
    };

    // For each entry, we create an embed field with the information
    let mut fields = vec![];
    for (category_name, favourites) in user_favourites.iter() {
        fields.push((category_name, format!("Total: {}", favourites.len()), true))
    }
    embed.fields(fields);

    // Message resolved, send it!
    ctx.send(|builder| {
        builder.embed(|e| {
            *e = embed;
            e
        })
    })
    .await?;

    Ok(())
}
