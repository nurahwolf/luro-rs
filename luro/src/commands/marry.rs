use crate::interaction::LuroSlash;

use std::fmt::Write;
use std::time::SystemTime;

use anyhow::Context;

use luro_builder::embed::EmbedBuilder;
use luro_model::user_marriages::UserMarriages;
use rand::seq::SliceRandom;
use rand::thread_rng;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::application::interaction::message_component::MessageComponentInteractionData;
use twilight_model::channel::message::component::{ActionRow, Button, ButtonStyle};
use twilight_model::channel::message::Component;

use crate::models::SlashUser;

use crate::luro_command::LuroCommand;

/// An array of reasons someone would like to marry.
/// TODO: Load this from disk once it's big enough
const MARRIAGE_REASONS: [&str; 2] = [
    "Hey <user>!\n\nIt looks like <author> finally felt it's time to confess their love to you, and have lowered themselves down to you to propose! Do you accept?",
    "*<author> just opened a box and presented <user> with a shiny tungsten ring! It looks like they want to get closer to each other. Do they accept?*",
];

#[derive(CommandModel, CreateCommand)]
#[command(name = "marry", desc = "Marry a user! Or see who you have married <3")]
pub enum MarryCommands {
    #[command(name = "someone")]
    New(MarryNew),
    #[command(name = "marriages")]
    Marriages(MarryMarriages)
}

impl LuroCommand for MarryCommands {
    async fn run_commands(self, ctx: LuroSlash) -> anyhow::Result<()> {
        // Call the appropriate subcommand.
        match self {
            Self::New(command) => command.run_command(ctx).await,
            Self::Marriages(command) => command.run_command(ctx).await
        }
    }

    async fn handle_component(self, data: Box<MessageComponentInteractionData>, ctx: LuroSlash) -> anyhow::Result<()> {
        let (marry, reason) = match self {
            Self::New(command) => (command.marry, command.reason),
            Self::Marriages(_) => return ctx.unknown_command_response().await
        };

        let interaction_author = ctx
            .interaction
            .author_id()
            .context("Expected interaction author to be present")?;

        match interaction_author == marry.resolved.id {
            false => {
                ctx.respond(|respond| {
                    respond.content(format!(
                        "Bruh. <@{}> just attempted to snipe the marriage.",
                        &interaction_author
                    ))
                })
                .await
            }
            true => {
                if &data.custom_id == "marry-deny" {
                    return ctx
                        .respond(|response| {
                            response
                                .content(format!(
                                    "It looks like <@{}> will never know what true love is like...",
                                    &marry.resolved.id
                                ))
                                .update()
                                .components(|c| c)
                        })
                        .await;
                }

                // Modify the proposer
                {
                    let mut user_data = ctx.framework.database.get_user(&interaction_author).await?;
                    user_data.marriages.insert(
                        marry.resolved.id,
                        UserMarriages {
                            timestamp: SystemTime::now(),
                            user: marry.resolved.id,
                            reason: reason.clone()
                        }
                    );
                }

                // Modify the proposee
                {
                    let mut user_data = ctx.framework.database.get_user(&marry.resolved.id).await?;
                    user_data.marriages.insert(
                        interaction_author,
                        UserMarriages {
                            timestamp: SystemTime::now(),
                            user: interaction_author,
                            reason: reason.clone()
                        }
                    );
                }

                ctx.respond(|response| {
                    response
                        .content(format!(
                            "Congratulations <@{}> & <@{}>!!!",
                            &interaction_author, &marry.resolved.id
                        ))
                        .components(|c| c)
                        .update()
                })
                .await
            }
        }
    }
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "marriages", desc = "Fetches someones marriages")]
pub struct MarryMarriages {
    /// Set this if you want to see someone elses marriages!
    user: Option<ResolvedUser>
}

impl LuroCommand for MarryMarriages {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let mut content = String::new();
        let (_, slash_author) = ctx.get_specified_user_or_author(&self.user, &ctx.interaction)?;
        let user_data = ctx.framework.database.get_user(&slash_author.user_id).await?;
        let mut embed = EmbedBuilder::default();
        embed
            .author(|author| {
                author
                    .name(format!("{}'s marriages", slash_author.name))
                    .icon_url(slash_author.avatar)
            })
            .colour(ctx.accent_colour().await);

        for (_, marriage) in user_data.marriages.iter() {
            match ctx.framework.twilight_cache.user(marriage.user) {
                Some(marriage_user) => writeln!(
                    content,
                    "{} - <@{}>```{}```",
                    marriage_user.name, marriage.user, marriage.reason
                )?,
                None => writeln!(content, "<@{}>\n```{}```", marriage.user, marriage.reason)?
            }
        }
        match content.is_empty() {
            true => embed.description("Looks like they have no marriages yet :("),
            false => embed.description(content)
        };

        ctx.respond(|r| r.add_embed(embed)).await
    }
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "someone", desc = "Propose to someone! So lucky, aww~")]
pub struct MarryNew {
    /// Set this if you want to marry someone!
    marry: ResolvedUser,
    /// The reason you wish to marry them!
    reason: String
}

impl LuroCommand for MarryNew {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let slash_author = SlashUser::client_fetch(
            &ctx.framework,
            ctx.interaction.guild_id,
            ctx.interaction
                .author_id()
                .context("Expected to find user who invoked command")?
        )
        .await?;
        let mut embed = EmbedBuilder::default();
        embed
            .author(|author| {
                author
                    .name(format!("{} has marriproposed!", slash_author.name))
                    .icon_url(slash_author.avatar)
            })
            .colour(ctx.accent_colour().await);

        {
            let mut rng = thread_rng();
            let reason = MARRIAGE_REASONS
                .choose(&mut rng)
                .context("Expected to be able to choose a random reason")?
                .replace("<user>", &format!("<@{}>", &self.marry.resolved.id))
                .replace("<author>", &format!("<@{}>", &slash_author.user_id));
            embed.description(reason);
        }

        embed.field(|f| f.field("Their Reason", &self.reason, true));
        ctx.respond(|r| {
            r.add_embed(embed)
                .content(format!("<@{}>", &self.marry.resolved.id))
                .add_components(buttons())
        })
        .await
    }
}

/// create components
fn buttons() -> Vec<Component> {
    vec![Component::ActionRow(ActionRow {
        components: vec![
            Component::Button(Button {
                custom_id: Some("marry-accept".to_owned()),
                disabled: false,
                emoji: None,
                label: Some("Do you accept?".to_owned()),
                style: ButtonStyle::Primary,
                url: None
            }),
            Component::Button(Button {
                custom_id: Some("marry-deny".to_owned()),
                disabled: false,
                emoji: None,
                label: Some("Do you deny?".to_owned()),
                style: ButtonStyle::Danger,
                url: None
            }),
        ]
    })]
}
