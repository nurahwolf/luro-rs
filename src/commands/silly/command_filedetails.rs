use poise::serenity_prelude::Attachment;

use crate::{Context, Error};

/// View the difference between two file sizes
#[poise::command(prefix_command, slash_command, category = "Silly")]
pub async fn file_details(
    ctx: Context<'_>,
    #[description = "File to examine"] file: Attachment,
    #[description = "Second file to examine"] file_2: Option<Attachment>
) -> Result<(), Error> {
    ctx.say(format!(
        "First file name: **{}**. File size difference: **{}** bytes",
        file.filename,
        file.size - file_2.map_or(0, |f| f.size)
    ))
    .await?;
    Ok(())
}
