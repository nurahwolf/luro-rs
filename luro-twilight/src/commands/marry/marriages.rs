use luro_framework::{CommandInteraction, Luro, LuroCommand};
use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::id::Id;

#[derive(CommandModel, CreateCommand)]
#[command(name = "marriages", desc = "Fetches someones marriages")]
pub struct Marriages {
    /// Set this if you want to see someone elses marriages!
    user: Option<ResolvedUser>,
}

impl LuroCommand for Marriages {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let accent_colour = ctx.accent_colour();
        let author = ctx.get_specified_user_or_author(self.user.as_ref()).await?;
        let marriages = author.fetch_marriages(ctx.database.clone()).await?;

        let mut marriages_detailed = vec![];
        let mut rejected_proposals = 0;
        let mut marriages_ended = 0;

        for marriage in marriages {
            if marriage.rejected {
                rejected_proposals += 1;
                continue;
            }

            if marriage.divorced {
                marriages_ended += 1;
                continue;
            }

            let proposer = ctx.fetch_user(Id::new(marriage.proposer_id as u64)).await?;
            let proposee = ctx.fetch_user(Id::new(marriage.proposee_id as u64)).await?;
            let approvers = ctx
                .database
                .count_marriage_approvers(marriage.proposer_id, marriage.proposee_id)
                .await?;

            marriages_detailed.push((marriage, proposer, proposee, approvers))
        }

        ctx.respond(|response| {
            response.embed(|embed| {
                embed
                    .title(format!("{}'s marriages | {} total", author.name(), marriages_detailed.len()))
                    .thumbnail(|t| t.url(author.avatar_url()))
                    .colour(accent_colour);

                if marriages_detailed.is_empty() {
                    embed.description("Looks like they have no marriages yet :(");
                }

                if marriages_ended != 0 {
                    embed.create_field("Ended Marriages", &format!("A total of `{}` time(s)", marriages_ended), true);
                }

                if rejected_proposals != 0 {
                    embed.create_field("Rejected Total", &format!("Rejected `{}` time(s)", rejected_proposals), true);
                }

                match marriages_detailed.len() < 25 {
                    true => {
                        for marriage in marriages_detailed {
                            let mut status = String::new();

                            if let Some(approvers) = marriage.3.approvers && approvers != 0 {
                                write!(status, " | {approvers} Approvals").unwrap();
                            }

                            if let Some(disapprovers) = marriage.3.disapprovers && disapprovers != 0 {
                                write!(status, " | {disapprovers} Disapprovals").unwrap();
                            }

                            if marriage.0.divorced {
                                write!(status, " | Devorced").unwrap();
                            }

                            if marriage.0.rejected {
                                write!(status, " | Rejected").unwrap();
                            }

                            embed.create_field(
                                &format!("{} and {}{status}", marriage.1.name(), marriage.2.name()),
                                &marriage.0.reason,
                                false,
                            );
                        }
                    }
                    false => {
                        let mut description = String::new();
                        for marriage in marriages_detailed {
                            writeln!(
                                description,
                                "- {} and {}\n  - {}",
                                marriage.1.name(),
                                marriage.2.name(),
                                marriage.0.reason
                            )
                            .unwrap();
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
