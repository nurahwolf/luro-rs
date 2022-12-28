use crate::{commands::testing, Context, Error, DATA_PATH};

use futures::{Stream, StreamExt};
use poise::serenity_prelude::{AttachmentType, Cache, CacheHttp};
use rand::seq::SliceRandom;
use std::fmt::Write;
use std::sync::Arc;
use std::{
    path::{Path, PathBuf},
    vec
};
use tokio::fs::{DirEntry, ReadDir};

async fn get_fursonas() -> Vec<String> {
    let mut fursona_names = vec![];

    if let Ok(mut dir) = tokio::fs::read_dir(Path::new("data/fursona/")).await {
        while let Ok(entry) = dir.next_entry().await {
            if let Some(directory) = entry {
                let test = directory.path();

                if test.is_dir() {
                    if let Ok(directory_name_string) = directory.file_name().into_string() {
                        fursona_names.push(directory_name_string.clone());
                    }
                }
            }
        }
    };

    if fursona_names.is_empty() {
        panic!("We fucked up, sorry");
    }

    fursona_names
}

async fn autocomplete_fursona<'a>(_ctx: Context<'_>, partial: &'a str) -> impl Stream<Item = String> + 'a {
    let fursona_directories = if let Ok(test) = folders(Path::new("data/fursona/")) {
        test
    } else {
        panic!("It's fucked!");
    };

    let test = directory(fursona_directories);

    futures::stream::iter(test).filter(move |name| futures::future::ready(name.starts_with(partial))).map(|name| name)
}

fn directory(fursona_directories: Vec<PathBuf>) -> Vec<String> {
    let mut fursona_names = vec![];
    for directory in fursona_directories {
        let filename = directory.file_name().unwrap().to_str().unwrap().to_string();
        fursona_names.push(filename.clone());
    }
    fursona_names
}

fn folders(dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    Ok(std::fs::read_dir(dir)?
        .into_iter()
        .filter(|r| r.is_ok()) // Get rid of Err variants for Result<DirEntry>
        .map(|r| r.unwrap().path()) // This is safe, since we only have the Ok variants
        .filter(|r| r.is_dir()) // Filter out non-folders
        .collect())
}

fn nsfw_check(ctx: Context) -> bool {
    if let Some(cache) = ctx.cache() {
        if let Some(channel) = cache.channel(ctx.channel_id()) {
            channel.is_nsfw()
        } else {
            false
        }
    } else {
        false
    }
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

    if nsfw_toggle && nsfw_toggle != nsfw_check(ctx) {
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
        ctx.say("I could not find any files in my backend. Did you specify the right fursona name?").await?;
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
