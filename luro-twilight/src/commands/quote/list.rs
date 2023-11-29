use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "list", desc = "List some quotes!")]
pub struct List {}

impl luro_framework::LuroCommand for List {
    async fn interaction_command(self, ctx: luro_framework::CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let quotes = ctx.database.driver.quotes_fetch().await?;

        ctx.respond(|r| {
            r.content(format!(
                "There are `{}` SFW quotes and `{}` NSFW quotes in the database!!",
                quotes.0.len(),
                quotes.1.len()
            ))
        })
        .await

        // let mut quotes_string = String::new();

        // for (id, quote) in quotes.into_iter() {
        //     let content = quote.content;
        //     if let Some(content) = content.lines().next() {
        //         let mut content = content.to_string();
        //         content = match quote.source {
        //             LuroMessageSource::Custom => format!("\n- `{id} (Custom)` - {content}"),
        //             _ => format!("\n- `{id}` - {content}"),
        //         };
        //         if content.len() > 75 {
        //             let mut split = 75;
        //             let mut success = false;
        //             while !success {
        //                 match content.is_char_boundary(split) {
        //                     true => {
        //                         content.truncate(split);
        //                         success = true
        //                     }
        //                     false => split += 1,
        //                 }
        //             }
        //             content.push_str("...");
        //         }
        //         quotes_string.push_str(&content);
        //     }
        // }

        // let accent_colour = interaction.accent_colour(&ctx).await;
        // interaction
        //     .respond(&ctx, |response| {
        //         response.embed(|embed| {
        //             embed
        //                 .colour(accent_colour)
        //                 .title("Some quotes to choose from...")
        //                 .description(quotes_string)
        //         })
        //     })
        //     .await
    }
}
