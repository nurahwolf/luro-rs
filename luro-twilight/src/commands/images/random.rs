use anyhow::Context;
use luro_framework::{CommandInteraction, Luro, LuroCommand};
use rand::{seq::SliceRandom, thread_rng};
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "random", desc = "Get some random images from my database!")]
pub struct Random {
    /// Should I get lewd?
    nsfw: bool,
}

impl LuroCommand for Random {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let accent_colour = ctx.accent_colour();

        let images = ctx.database.images_fetch(self.nsfw).await?;
        let image = images.choose(&mut thread_rng()).context("There are no images in the database.")?;
        let image_owner = ctx.fetch_user(twilight_model::id::Id::new(image.owner_id as u64)).await?;

        ctx.respond(|r| {
            r.embed(|e| {
                if let Some(source) = image.source.clone() {
                    e.url(source);
                }
                e.colour(accent_colour)
                    .title(image.name.clone())
                    .image(|i| i.url(image.url.clone()))
                    .footer(|f| {
                        f.text(format!(
                            "{} | Image ID: {}",
                            match self.nsfw {
                                true => "NSFW",
                                false => "SFW",
                            },
                            image.img_id
                        ))
                    })
                    .author(|author| {
                        author
                            .name(format!("Image by {}", image_owner.name()))
                            .icon_url(image_owner.avatar_url())
                    })
            })
        })
        .await
    }
}
