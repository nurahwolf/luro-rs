use std::convert::TryInto;
use std::fmt::Write;
use std::time::SystemTime;

use anyhow::Context;
use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::application::interaction::message_component::MessageComponentInteractionData;
use twilight_model::channel::message::component::{ActionRow, Button, ButtonStyle};
use twilight_model::channel::message::Component;

use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedFieldBuilder};

use crate::models::{LuroCommandCache, LuroSlash, SlashUser, UserData, UserMarriages};

use crate::traits::luro_command::LuroCommand;
use crate::traits::luro_functions::LuroFunctions;

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
    async fn run_commands(self, ctx: LuroSlash) -> anyhow::Result<()> {
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
    async fn run_command(self, mut ctx: LuroSlash) -> anyhow::Result<()> {
        let mut content = String::new();
        let (_, slash_author) = ctx.get_specified_user_or_author(&self.user, &ctx.interaction)?;
        let user_data = UserData::get_user_settings(&ctx.luro, &slash_author.user_id).await?;
        let embed_author =
            EmbedAuthorBuilder::new(format!("{}'s marriages", slash_author.name)).icon_url(slash_author.try_into()?);
        let mut embed = ctx.default_embed().await?.author(embed_author);

        for (_, marriage) in user_data.marriages.iter() {
            match ctx.luro.twilight_cache.user(marriage.user) {
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
    async fn run_command(self, mut ctx: LuroSlash) -> anyhow::Result<()> {
        let slash_author = SlashUser::client_fetch(
            &ctx.luro,
            ctx.interaction.guild_id,
            ctx.interaction
                .author_id()
                .context("Expected to find user who invoked command")?
        )
        .await?;
        let embed_author =
            EmbedAuthorBuilder::new(format!("{} has proposed!", &slash_author.name)).icon_url(slash_author.clone().try_into()?);
        let mut embed = ctx.default_embed().await?.author(embed_author);

        embed = embed.description(format!("**Hey <@{}>!**\n\nIt looks like <@{}> finally felt it's time to confess their love to you, and have lowered themselves down to you to propose! Do you accept?", self.marry.resolved.id, &slash_author.user_id));
        embed = embed.field(EmbedFieldBuilder::new("Their Reason", self.reason.clone()));
        ctx.components(button("marry", "Do you accept?"));

        ctx.embed(embed.build())?
            .content(format!("<@{}>", self.marry.resolved.id.clone()))
            .respond()
            .await?;

        let response = ctx.interaction_client().response(&ctx.interaction.token).await?.model().await?;

        ctx.luro.command_cache.insert(
            response.id,
            LuroCommandCache {
                author: slash_author.user_id,
                user_in_command: self.marry.resolved.id,
                reason: self.reason.clone()
            }
        );

        Ok(())
    }

    async fn handle_component(_: MessageComponentInteractionData, mut ctx: LuroSlash) -> anyhow::Result<()> {
        let interaction_author = ctx.author()?;
        let message = match ctx.interaction.message.clone() {
            Some(message) => message,
            None => {
                return ctx
                    .content("Could not check if you can marry - Original interaction not found")
                    .ephemeral()
                    .respond()
                    .await
            }
        };

        let command_data = match ctx.clone().luro.command_cache.get(&message.id) {
            Some(command_data) => command_data.clone(),
            None => {
                return ctx
                    .content("Could not check if you can marry - No data in my cache")
                    .ephemeral()
                    .respond()
                    .await
            }
        };

        match interaction_author.id == command_data.user_in_command {
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
                    let mut user_data = UserData::modify_user_settings(&ctx.luro, &command_data.author).await?;
                    user_data.marriages.insert(
                        command_data.user_in_command,
                        UserMarriages {
                            timestamp: SystemTime::now(),
                            user: command_data.user_in_command,
                            reason: command_data.reason.clone()
                        }
                    );
                }

                // Modify the proposee
                {
                    let mut user_data = UserData::modify_user_settings(&ctx.luro, &command_data.user_in_command).await?;
                    user_data.marriages.insert(
                        command_data.author,
                        UserMarriages {
                            timestamp: SystemTime::now(),
                            user: command_data.author,
                            reason: command_data.reason.clone()
                        }
                    );
                }


                ctx.content(format!(
                    "Congratulations <@{}> & <@{}>!!!",
                    &command_data.author, &command_data.user_in_command
                )).components(vec![]).update()
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
