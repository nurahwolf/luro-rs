use luro_framework::{CommandInteraction, Luro, LuroCommand};
use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::id::{marker::UserMarker, Id};

#[derive(CommandModel, CreateCommand)]
#[command(name = "marriages", desc = "Fetches someones marriages")]
pub struct Marriages {
    /// Set this if you want to see someone elses marriages!
    user: Option<Id<UserMarker>>,
}

impl LuroCommand for Marriages {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let accent_colour = ctx.accent_colour();
        let author = ctx.get_specified_user_or_author(self.user).await?;
        let marriages = ctx.database.user_fetch_marriages(author.user_id).await?;

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

            let proposer = ctx.fetch_user(marriage.proposer_id).await?;
            let proposee = ctx.fetch_user(marriage.proposee_id).await?;
            let all_approvals = ctx
                .database
                .driver
                .marriage_fetch_approvals(marriage.proposer_id, marriage.proposee_id)
                .await?;

            let mut approvals = 0;
            let mut disapprovals = 0;
            for approval in all_approvals {
                if approval.approve {
                    approvals += 1;
                }
                if approval.disapprove {
                    disapprovals += 1;
                }
            }

            marriages_detailed.push((marriage, proposer, proposee, approvals, disapprovals))
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

                            if marriage.3 != 0 {
                                write!(status, " | {} Approvals", marriage.3).unwrap();
                            }

                            if marriage.4 != 0 {
                                write!(status, " | {} Disapprovals", marriage.4).unwrap();
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
