use crate::commands::anyhow;
use std::convert::TryInto;
use std::fmt::Write;
use std::mem;
use std::time::SystemTime;

use anyhow::Context;
use async_trait::async_trait;

use luro_model::user_marriages::UserMarriages;
use rand::seq::SliceRandom;
use rand::thread_rng;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::application::interaction::message_component::MessageComponentInteractionData;
use twilight_model::application::interaction::InteractionData;
use twilight_model::channel::message::component::{ActionRow, Button, ButtonStyle};
use twilight_model::channel::message::Component;

use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedFieldBuilder};

use crate::models::SlashUser;

use crate::slash::Slash;
use crate::traits::luro_command::LuroCommand;
use crate::traits::luro_functions::LuroFunctions;

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

#[async_trait]
impl LuroCommand for MarryCommands {
    async fn run_commands(self, ctx: Slash) -> anyhow::Result<()> {
        // Call the appropriate subcommand.
        match self {
            Self::New(command) => command.run_command(ctx).await,
            Self::Marriages(command) => command.run_command(ctx).await
        }
    }
}

#[derive(CommandModel, CreateCommand)]
#[command(name = "marriages", desc = "Fetches someones marriages")]
pub struct MarryMarriages {
    /// Set this if you want to see someone elses marriages!
    user: Option<ResolvedUser>
}

#[async_trait]
impl LuroCommand for MarryMarriages {
    async fn run_command(self, mut ctx: Slash) -> anyhow::Result<()> {
        let mut content = String::new();
        let (_, slash_author) = ctx.get_specified_user_or_author(&self.user, &ctx.interaction)?;
        let user_data = ctx.framework.database.get_user(&slash_author.user_id).await?;
        let embed_author =
            EmbedAuthorBuilder::new(format!("{}'s marriages", slash_author.name)).icon_url(slash_author.try_into()?);
        let mut embed = ctx.default_embed().await?.author(embed_author);

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
            true => embed = embed.description("Looks like they have no marriages yet :("),
            false => embed = embed.description(content)
        }

        ctx.embed(embed.build())?.respond().await
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

#[async_trait]
impl LuroCommand for MarryNew {
    async fn run_command(self, mut ctx: Slash) -> anyhow::Result<()> {
        let slash_author = SlashUser::client_fetch(
            &ctx.framework,
            ctx.interaction.guild_id,
            ctx.interaction
                .author_id()
                .context("Expected to find user who invoked command")?
        )
        .await?;
        let embed_author =
            EmbedAuthorBuilder::new(format!("{} has proposed!", &slash_author.name)).icon_url(slash_author.clone().try_into()?);
        let mut embed = ctx.default_embed().await?.author(embed_author);

        {
            let mut rng = thread_rng();
            let reason = MARRIAGE_REASONS
                .choose(&mut rng)
                .context("Expected to be able to choose a random reason")?
                .replace("<user>", &format!("<@{}>", &self.marry.resolved.id))
                .replace("<author>", &format!("<@{}>", &slash_author.user_id));
            embed = embed.description(reason);
        }

        embed = embed.field(EmbedFieldBuilder::new("Their Reason", self.reason.clone()));
        ctx.components(button("marry", "Do you accept?"));

        ctx.embed(embed.build())?
            .content(format!("<@{}>", self.marry.resolved.id.clone()))
            .respond()
            .await?;

        let response = ctx
            .interaction_client()
            .response(&ctx.interaction.token)
            .await?
            .model()
            .await?;

        ctx.framework.database.command_data.insert(response.id, ctx.interaction);

        Ok(())
    }

    async fn handle_component(_: Box<MessageComponentInteractionData>, mut ctx: Slash) -> anyhow::Result<()> {
        let interaction_author = ctx.author()?;
        let _message = match ctx.interaction.message.clone() {
            Some(message) => message,
            None => {
                return ctx
                    .content("Could not check if you can marry - Original interaction not found")
                    .ephemeral()
                    .respond()
                    .await
            }
        };

        let old_interaction = match mem::take(&mut ctx.interaction.data) {
            Some(InteractionData::ApplicationCommand(data)) => Self::new(*data).await?,
            _ => return Err(anyhow!("unable to parse modal data, received unknown data type"))
        };

        match interaction_author.id == old_interaction.marry.resolved.id {
            false => {
                ctx.content(format!(
                    "Bruh. <@{}> just attempted to snipe the marriage.",
                    &interaction_author.id
                ))
                .respond()
                .await
            }
            true => {
                // Modify the proposer
                {
                    let mut user_data = ctx.framework.database.get_user(&interaction_author.id).await?;
                    user_data.marriages.insert(
                        old_interaction.marry.resolved.id,
                        UserMarriages {
                            timestamp: SystemTime::now(),
                            user: old_interaction.marry.resolved.id,
                            reason: old_interaction.reason.clone()
                        }
                    );
                }

                // Modify the proposee
                {
                    let mut user_data = ctx.framework.database.get_user(&old_interaction.marry.resolved.id).await?;
                    user_data.marriages.insert(
                        interaction_author.id,
                        UserMarriages {
                            timestamp: SystemTime::now(),
                            user: interaction_author.id,
                            reason: old_interaction.reason.clone()
                        }
                    );
                }

                ctx.content(format!(
                    "Congratulations <@{}> & <@{}>!!!",
                    &interaction_author.id, &old_interaction.marry.resolved.id
                ))
                .components(vec![])
                .update()
                .respond()
                .await
            }
        }
    }
}

/// Return a button
fn button(custom_id: impl Into<String>, label: impl Into<String>) -> Vec<Component> {
    Vec::from([Component::ActionRow(ActionRow {
        components: Vec::from([Component::Button(Button {
            custom_id: Some(custom_id.into()),
            disabled: false,
            emoji: None,
            label: Some(label.into()),
            style: ButtonStyle::Primary,
            url: None
        })])
    })])
}
