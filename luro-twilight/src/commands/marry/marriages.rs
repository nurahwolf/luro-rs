use std::fmt::Write;
use luro_framework::{command::ExecuteLuroCommand, interactions::InteractionTrait, CommandInteraction, Luro};
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

#[derive(CommandModel, CreateCommand)]
#[command(name = "marriages", desc = "Fetches someones marriages")]
pub struct Marriages {
    /// Set this if you want to see someone elses marriages!
    user: Option<ResolvedUser>,
}

impl ExecuteLuroCommand for Marriages {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let accent_colour = ctx.accent_colour().await;
        let user = ctx.get_user(&self.user.map(|x| x.resolved.id).unwrap_or(ctx.author_id())).await?;
        let marriages = ctx.database.get_marriages(user.id.get() as i64).await?;

        ctx.respond(|response| {
            response.embed(|embed| {
                embed
                    .title(format!("{}'s marriages", user.name()))
                    .thumbnail(|t|t.url(user.avatar()))
                    .colour(accent_colour);

                if marriages.is_empty() {
                    embed.description("Looks like they have no marriages yet :(");
                }

                match marriages.len() < 25 {
                    true => {
                        for marriage in marriages {
                            embed.create_field(&user.name, &marriage.reason, false);
                        }
                    }
                    false => {
                        let mut description = String::new();
                        for marriage in marriages {
                            writeln!(description, "- {} - <@{}>\n  - {}", marriage.reason, user.name, user.id).unwrap();
                        }
                        embed.description(description);
                    }
                }

                embed
            })
        })
        .await
    }
}
