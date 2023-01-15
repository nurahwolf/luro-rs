use luro_core::{stories::Stories, STORIES_FILE_PATH};
use luro_utilities::guild_accent_colour;

use luro_core::{Context, Error};use rand::Rng;

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx]
    }
}

/// A random story... What could go wrong?
#[poise::command(slash_command, prefix_command, category = "Quotes")]
pub async fn story(
    ctx: Context<'_>,
    #[description = "Send the story as just text"]
    #[flag]
    plain_text: bool,
    #[description = "The number story to get"] story: Option<usize>
) -> Result<(), Error> {
    // Load the story mutex, reload from config if its empty
    let accent_colour = ctx.data().config.read().await.accent_colour;
    let mut stories = ctx.data().stories.write().await;
    if stories.stories.is_empty() {
        ctx.say("Out of random stories to get, so reloading config...").await?;
        stories.reload(&Stories::get(STORIES_FILE_PATH).await);
    }
    // Generate a random number based on the length of the stories vec
    let random_number = rand::thread_rng().gen_range(0..stories.stories.len());

    // If the user specified a story, get it.
    let story_resolved = if let Some(index) = story {
        let stories = Stories::get(STORIES_FILE_PATH).await.stories;
        match stories.get(index) {
            Some(story) => story.clone(),
            None => {
                ctx.say("Failed to find that quote!").await?;
                return Ok(());
            }
        }
    } else {
        stories.stories.remove(random_number)
    };

    if !plain_text {
        let story_shortened = truncate(&story_resolved[1], 4096);
        ctx.send(|b| {
            b.embed(|b| {
                b.title(&story_resolved[0])
                    .description(story_shortened)
                    .color(guild_accent_colour(accent_colour, ctx.guild()))
                    .footer(|f| f.text(format!("Story ID: {}", story.unwrap_or(random_number))))
            })
        })
        .await?;
    } else if story_resolved[1].len() > 2048 {
        let (split1, split2) = story_resolved[1].split_at(2000);
        ctx.say(split1).await?;
        ctx.say(split2).await?;
    } else {
        ctx.say(&story_resolved[1]).await?;
    }

    Ok(())
}
