use anyhow::Context;
use luro_database::LuroCharacterImage;
use luro_framework::{CommandInteraction, Luro, LuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "add", desc = "Set the primary image for this character")]
pub struct Add {
    #[command(desc = "The character to get", autocomplete = true)]
    character: String,
    /// The name of the image
    name: String,
    /// The URL the image should be set to
    url: String,
    /// Is this a NSFW image?
    nsfw: bool,
    /// Do you want this image to show up as one of the main images in your profile?
    fav: bool,
    /// Overwrite an image by ID
    overwrite: Option<i64>,
    /// The source URL for the image
    source: Option<String>,
}

impl LuroCommand for Add {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let character = ctx
            .author
            .fetch_character(ctx.database.clone(), &self.character)
            .await?
            .context("No character available")?;

        let img = LuroCharacterImage {
            img_id: self.overwrite.unwrap_or_default(),
            name: self.name,
            nsfw: self.nsfw,
            owner_id: ctx.author.user_id,
            source: self.source,
            url: self.url,
            character_name: self.character,
            favourite: self.fav,
        };

        match self.overwrite {
            Some(_) => character.update_image(&img).await?,
            None => character.new_image(&img).await?,
        };

        let mut embed = ctx.default_embed().await;
        embed.footer(|f| f.text(format!("Image ID: {}", img.img_id)))
            .image(|i| i.url(img.url))
            .author(|author| {
                author
                    .name(format!("Profile by {}", ctx.author.name()))
                    .icon_url(ctx.author.avatar_url())
        });

        if let Some(source) = &img.source {
            embed.url(source);
        }

        if !img.name.is_empty() {
            embed.title(img.name);
        }

        ctx.respond(|r| r.add_embed(embed).ephemeral()).await
    }
}
