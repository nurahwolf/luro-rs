use crate::{utils::guild_accent_colour, Context, Error};
use rand::Rng;

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
    let stories = &ctx.data().stories.stories;
    let random_number = rand::thread_rng().gen_range(0..stories.len());

    // Try to get the specified quote
    if let Some(story) = story {
        if let Some(story_result) = stories.get(story) {
            let story_shortened = truncate(&story_result[1], 4096);
            ctx.send(|b| {
                b.embed(|b| {
                    b.title(&story_result[0])
                        .description(story_shortened)
                        .color(guild_accent_colour(ctx.data().config.accent_colour, ctx.guild()))
                        .footer(|f| f.text(format!("Story ID: {story}")))
                })
            })
            .await?;
            return Ok(());
        } else {
            ctx.say("Failed to get that story! Sure you got the right ID?").await?;
            return Ok(());
        }
    }

    if let Some(story) = stories.get(random_number) {
        if !plain_text {
            let story_shortened = truncate(&story[1], 4096);
            ctx.send(|b| {
                b.embed(|b| {
                    b.title(&story[0])
                        .description(story_shortened)
                        .color(guild_accent_colour(ctx.data().config.accent_colour, ctx.guild()))
                        .footer(|f| f.text(format!("Story ID: {random_number}")))
                })
            })
            .await?;
        } else if story[1].len() > 2048 {
            let (split1, split2) = story[1].split_at(2000);
            ctx.say(split1).await?;
            ctx.say(split2).await?;
        } else {
            ctx.say(&story[1]).await?;
        }
    } else {
        ctx.say("Failed to find a story ;c").await?;
    }

    Ok(())
}
