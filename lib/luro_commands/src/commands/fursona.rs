use futures::{Stream, StreamExt};
use luro_core::Context;
use luro_core::Error;
use luro_core::DATA_PATH;
use luro_core::FURSONA_FILE_PATH;

use luro_utilities::nsfw_check;
use poise::serenity_prelude::AttachmentType;
use poise::serenity_prelude::CacheHttp;
use rand::seq::SliceRandom;

use std::{path::Path, vec};
use tokio::fs::DirEntry;

async fn get_fursonas() -> Result<Vec<String>, Error> {
    let mut fursona_names = vec![];
    let mut dir = match tokio::fs::read_dir(Path::new(FURSONA_FILE_PATH)).await {
        Ok(dir) => dir,
        Err(_) => panic!("Failed to read fursona directory (does it exist?)")
    };

    while let Some(entry) = dir.next_entry().await? {
        let directory_path = entry.path();

        if directory_path.is_dir() {
            let directory_name = match entry.file_name().into_string() {
                Ok(name) => name,
                Err(_) => panic!("Failed to read directory into string")
            };
            fursona_names.push(directory_name);
        }
    }

    if fursona_names.is_empty() {
        panic!("Failed to get the names within the fursona directory (does it exist?)");
    }

    Ok(fursona_names)
}

async fn autocomplete_fursona<'a>(_ctx: Context<'_>, partial: &'a str) -> impl Stream<Item = String> + 'a {
    let fursonas = match get_fursonas().await {
        Ok(fursonas) => fursonas,
        Err(err) => panic!("Fursona: Failed to get fursonas - {err}")
    };

    futures::stream::iter(fursonas)
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
        .map(|name| name)
}

/// Random images of someones fursona! Try out `nurah`!
#[poise::command(slash_command, prefix_command, category = "Furry")]
pub async fn fursona(
    ctx: Context<'_>,
    #[description = "Specify the fursona to get, such as 'nurah'!"]
    #[autocomplete = "autocomplete_fursona"]
    fursona_name: String,
    #[description = "Specify true to see naughty art ;)"]
    #[flag]
    nsfw_toggle: bool
) -> Result<(), Error> {
    // Someone is being naughty and trying to get NSFW in a SFW room... smh.

    if nsfw_toggle && nsfw_toggle != nsfw_check(ctx.cache(), ctx.channel_id()) {
        ctx.say("SMH. Are you really after NSFW art in a SFW channel you fucking degenerate? Reconsider your life choices.\nYes, I did just call you out in public.")
            .await?;
        return Ok(());
    }

    let mut files: Vec<DirEntry> = Vec::new();
    let search_path = if nsfw_toggle {
        format!("{DATA_PATH}/fursona/{fursona_name}/nsfw")
    } else {
        format!("{DATA_PATH}/fursona/{fursona_name}/sfw")
    };

    if let Ok(mut dir) = tokio::fs::read_dir(search_path).await {
        while let Some(entry) = dir.next_entry().await? {
            // Here, `entry` is a `DirEntry`.
            files.push(entry)
        }
    } else {
        ctx.say("Looks like that fursona does not have any art of that type. Sorry! (Usually happens on SFW only fursonas)")
            .await?;
        return Ok(());
    };

    if files.is_empty() {
        ctx.say("I could not find any files in my backend. Did you specify the right fursona name?")
            .await?;
        return Ok(());
    }

    let file = if let Some(file) = files.choose(&mut rand::thread_rng()) {
        file
    } else {
        return Ok(());
    };

    let file_tokio = if let Ok(file_tokio) = tokio::fs::File::open(file.path()).await {
        file_tokio
    } else {
        ctx.say("Failed to open the file with tokio (Backend error)").await?;
        return Ok(());
    };

    let file_name = if let Ok(file_name) = file.file_name().into_string() {
        file_name
    } else {
        ctx.say("Failed to get the file name of the image (Backend error)").await?;
        return Ok(());
    };

    if ctx
        .send(|builder| {
            builder
                .attachment(AttachmentType::File {
                    file: (&file_tokio),
                    filename: file_name.clone()
                })
                .content(file_name.clone())
        })
        .await
        .is_ok()
    {
        Ok(())
    } else {
        ctx.say(format!("Failed to send the image (This is usually because of Discord's bullshit 8MB filesize limit for bots. Just run me again and I shouldn't fail.)\nThe file I failed on was {file_name}")).await?;
        Ok(())
    }
}
