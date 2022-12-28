use poise::serenity_prelude::ButtonStyle;
use rand::{seq::IteratorRandom, thread_rng};
use reqwest::Client;

use crate::{
    commands::furry::struct_e621::{E621Post, E621Posts},
    Context, Error
};

/// TODO
fn random_remove(input: &mut Vec<E621Post>) -> Option<E621Post> {
    let i = (0..input.len()).choose(&mut thread_rng())?;
    Some(input.swap_remove(i))
}

/// Search e621 you lewd thing
#[poise::command(slash_command, prefix_command, nsfw_only, category = "Testing", reuse_response)]
pub async fn e621_tiled(
    ctx: Context<'_>,
    #[description = "Just send the image result"]
    #[flag]
    _just_image: bool,
    #[description = "Disable the default Blacklist"]
    #[flag]
    disable_blacklist: bool,
    #[description = "Show blacklisted tags"]
    #[flag]
    _show_blacklist: bool,
    #[description = "Show up to 4 images at once (desktop only)"]
    #[flag]
    _tiled: bool,
    #[description = "Search Term"]
    #[rest]
    search: String
) -> Result<(), Error> {
    // Bail if there is no search term
    if search.is_empty() {
        ctx.say("You did not provide something to search.").await?;
        return Ok(());
    }

    let mut query = search.clone();
    if disable_blacklist {
        query.push_str(ctx.data().config.lock().unwrap().e621_blacklist.clone().as_str())
    }

    let response = {
        let client = Client::builder().user_agent("Luro/1.0 (by nurahwolf on e621)").build()?;
        client
            .get("https://e621.net/posts.json?")
            .basic_auth("nurahwolf", ctx.data().secrets.e621_token.clone())
            .query(&[("tags", query)])
            .send()
            .await?
    };

    let e621_posts = response.json::<E621Posts>().await?;

    // Bail if no posts in response
    if e621_posts.posts.is_empty() {
        ctx.say("Your search returned fuck all.").await?;
        return Ok(());
    }

    let _test = ctx
        .send(|b| {
            b.content("Here we have message 1....")
                .components(|b| b.create_action_row(|b| b.create_button(|b| b.label("Click me! OwO").style(ButtonStyle::Primary).custom_id("message_1"))))
        })
        .await?;

    // let interaction = test.message().await?.await_component_interaction(ctx).timeout(Duration::from_secs(60 * 3)).await.unwrap();

    // match interaction.data.custom_id.as_str() {
    //     "message_2" => {
    //         interaction
    //             .create_interaction_response(ctx, |f| {
    //                 f.interaction_response_data(|f| {
    //                     f.content("Here we have message 2....")
    //                     .components(|b| b.create_action_row(|b| b.create_button(|b| b.label("Click me! OwO").style(serenity::ButtonStyle::Primary).custom_id("message_3"))))
    //                 })
    //             })
    //             .await?;
    //     }
    //     "message_3" => {
    //         interaction
    //             .create_interaction_response(ctx, |f| {
    //                 f.interaction_response_data(|f| {
    //                     f.content("Here we have message 3....")
    //                     .components(|b| b.create_action_row(|b| b.create_button(|b| b.label("Click me! OwO").style(serenity::ButtonStyle::Primary).custom_id("message_2"))))
    //                 })
    //             })
    //             .await?;
    //     }
    //     _ => {
    //         ctx.say("We had a fucky wucky sorry").await?;
    //     }
    // }

    //     // Wait for multiple interactions
    //     let mut interaction_stream = test.message().await?.await_component_interactions(ctx).timeout(Duration::from_secs(60 * 3)).build();

    //     while let Some(interaction) = interaction_stream.next().await {
    //         // Acknowledge the interaction and send a reply
    //         interaction
    //             .create_interaction_response(&ctx, |r| {
    //                 // This time we dont edit the message but reply to it
    //                 r.kind(InteractionResponseType::ChannelMessageWithSource)
    //                     .interaction_response_data(|d| {
    //                         // Make the message hidden for other users by setting `ephemeral(true)`.
    //                         d.ephemeral(true)
    //                             .content("Everything is on fire.")
    //                     })
    //             })
    //             .await?;
    //     }

    //     test.delete(ctx).await?;

    // if let Some(interaction) = test2.await_component_interaction(ctx).timeout(Duration::from_secs(60 * 3)).await {
    //     let test3 = &interaction.data.custom_id;

    // }

    // let mut embeds = vec![];
    // if tiled {
    //     for _ in 0..4 {
    //         let mut embed = CreateEmbed::default();

    //         if let Some(post) = random_remove(&mut e621_posts.posts) {
    //             embed.title(&search);
    //             embed.url("https://e621.net/posts/");
    //             embed.color(guild_accent_colour(ctx.guild()));
    //             embed.image(post.file.url);
    //             embed.description(post.description);
    //         }

    //         embeds.push(embed);
    //     }
    // }

    // ctx.send(|builder| {
    //     for embed in embeds {
    //         builder.embed(|f| {
    //             *f = embed;
    //             f
    //         });
    //     }
    //     builder
    // })
    // .await?;

    let image_url = "https://raw.githubusercontent.com/serenity-rs/serenity/current/logo.png";
    ctx.send(|b| {
        b.content("message 1")
            .embed(|b| b.description("embed 1").image(image_url))
            .components(|b| b.create_action_row(|b| b.create_button(|b| b.label("button 1").style(ButtonStyle::Primary).custom_id(1))))
    })
    .await?;

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let image_url = "https://raw.githubusercontent.com/serenity-rs/serenity/current/examples/e09_create_message_builder/ferris_eyes.png";
    ctx.send(|b| {
        b.content("message 2")
            .embed(|b| b.description("embed 2").image(image_url))
            .components(|b| b.create_action_row(|b| b.create_button(|b| b.label("button 2").style(ButtonStyle::Danger).custom_id(2))))
    })
    .await?;

    Ok(())
}
