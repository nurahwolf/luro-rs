use luro_database::LuroImage;
use luro_framework::{CommandInteraction, Luro, LuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "add", desc = "Add an image to the database!")]
pub struct Add {
    /// The name of the image
    name: String,
    /// The URL the image should be set to
    url: String,
    /// Is this a NSFW image?
    nsfw: bool,
    /// The source URL for the image
    source: Option<String>,
}

impl LuroCommand for Add {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let accent_colour = ctx.accent_colour();
        let img = ctx
            .database
            .new_image(LuroImage {
                img_id: 0,
                name: self.name,
                nsfw: self.nsfw,
                owner_id: ctx.author.user_id().get() as i64,
                source: self.source,
                url: self.url,
            })
            .await?;

        let image_owner = ctx.fetch_user(&twilight_model::id::Id::new(img.owner_id as u64)).await?;

        ctx.respond(|r| {
            r.embed(|e| {
                if let Some(source) = img.source.clone() {
                    e.url(source);
                }
                e.colour(accent_colour)
                    .title(img.name.clone())
                    .image(|i| i.url(img.url.clone()))
                    .footer(|f| {
                        f.text(format!(
                            "{} | Image ID: {}",
                            match self.nsfw {
                                true => "NSFW",
                                false => "SFW",
                            },
                            img.img_id
                        ))
                    })
                    .author(|author| {
                        author
                            .name(format!("Image by {}", image_owner.name()))
                            .icon_url(image_owner.avatar())
                    })
            })
        })
        .await
    }
}
