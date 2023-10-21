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
        let user = ctx.fetch_user(&ctx.author.user_id()).await?;
        let character = user
            .fetch_character(ctx.database.clone(), &self.character)
            .await?
            .context("Expected to get character")?;

        let img = character
            .new_image(LuroCharacterImage {
                img_id: 0,
                name: self.name,
                nsfw: self.nsfw,
                owner_id: user.user_id,
                source: self.source,
                url: self.url,
                character_name: self.character,
                favourite: self.fav,
            })
            .await?;

        let mut embed = ctx.default_embed().await;
        embed.footer(|f| f.text(format!("Image ID: {}", img.img_id)));

        if let Some(source) = &img.source {
            embed.url(source);
        }

        embed.image(|i| i.url(img.url));
        if !img.name.is_empty() {
            embed.title(img.name);
        }

        embed.author(|author| author.name(format!("Profile by {}", user.name())).icon_url(user.avatar()));

        ctx.respond(|r| r.add_embed(embed).ephemeral()).await
    }
}
