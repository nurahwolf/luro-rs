use std::convert::TryInto;
use std::fmt::Write;
use std::time::SystemTime;

use anyhow::Context;
use async_trait::async_trait;

use rand::seq::SliceRandom;
use rand::thread_rng;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::application::interaction::message_component::MessageComponentInteractionData;

use twilight_model::channel::message::component::{ActionRow, Button, ButtonStyle};
use twilight_model::channel::message::Component;

use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedFieldBuilder};

use crate::models::{LuroCommandCache, LuroResponse, SlashUser, UserData, UserMarriages};
use crate::LuroContext;

use crate::traits::luro_command::LuroCommand;

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
    async fn run_commands(self, ctx: &LuroContext, slash: LuroResponse) -> anyhow::Result<()> {
        // Call the appropriate subcommand.
        match self {
            Self::New(command) => command.run_command(ctx, slash).await,
            Self::Marriages(command) => command.run_command(ctx, slash).await
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
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let mut content = String::new();
        let (_, slash_author) = ctx.get_specified_user_or_author(&self.user, &slash)?;
        let user_data = UserData::get_user_settings(ctx, &slash_author.user_id).await?;
        let embed_author =
            EmbedAuthorBuilder::new(format!("{}'s marriages", slash_author.name)).icon_url(slash_author.try_into()?);
        let mut embed = ctx.default_embed(&slash.interaction.guild_id).author(embed_author);

        for (_, marriage) in user_data.marriages.iter() {
            match ctx.twilight_cache.user(marriage.user) {
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

        slash.embed(embed.build())?;
        ctx.respond(&mut slash).await
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
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let mut response = slash.clone();
        let slash_author = SlashUser::client_fetch(
            ctx,
            slash.interaction.guild_id,
            slash
                .interaction
                .author_id()
                .context("Expected to find user who invoked command")?
        )
        .await?;
        let embed_author =
            EmbedAuthorBuilder::new(format!("{} has proposed!", &slash_author.name)).icon_url(slash_author.clone().try_into()?);
        let mut embed = ctx.default_embed(&slash.interaction.guild_id).author(embed_author);

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
        response.components(button("marry", "Do you accept?"));

        response
            .embed(embed.build())?
            .content(format!("<@{}>", self.marry.resolved.id.clone()));

        ctx.respond(&mut slash).await?;

        let response = ctx
            .interaction_client(&slash)
            .response(&slash.interaction.token)
            .await?
            .model()
            .await?;

        ctx.data_command.insert(
            response.id,
            LuroCommandCache {
                author: slash_author.user_id,
                user_in_command: self.marry.resolved.id,
                reason: self.reason.clone()
            }
        );

        Ok(())
    }

    async fn handle_component(
        _: Box<MessageComponentInteractionData>,
        ctx: &LuroContext,
        slash: &mut LuroResponse
    ) -> anyhow::Result<()> {
        let slash_author = SlashUser::client_fetch(
            ctx,
            slash.interaction.guild_id,
            slash
                .interaction
                .author_id()
                .context("Expected to find user who invoked command")?
        )
        .await?;
        let message = match slash.interaction.message.clone() {
            Some(message) => message,
            None => {
                slash
                    .content("Could not check if you can marry - Original interaction not found")
                    .ephemeral();
                return ctx.respond(slash).await;
            }
        };

        let command_data = match ctx.data_command.get(&message.id) {
            Some(command_data) => command_data.clone(),
            None => {
                slash
                    .content("Could not check if you can marry - No data in my cache")
                    .ephemeral();
                return ctx.respond(slash).await;
            }
        };

        match slash_author.user_id == command_data.user_in_command {
            false => {
                slash.content(format!(
                    "Bruh. <@{}> just attempted to snipe the marriage.",
                    &slash_author.user_id
                ));

                ctx.respond(slash).await
            }
            true => {
                // Modify the proposer
                {
                    let mut user_data = UserData::modify_user_settings(ctx, &command_data.author).await?;
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
                    let mut user_data = UserData::modify_user_settings(ctx, &command_data.user_in_command).await?;
                    user_data.marriages.insert(
                        command_data.author,
                        UserMarriages {
                            timestamp: SystemTime::now(),
                            user: command_data.author,
                            reason: command_data.reason.clone()
                        }
                    );
                }

                slash
                    .content(format!(
                        "Congratulations <@{}> & <@{}>!!!",
                        &command_data.author, &command_data.user_in_command
                    ))
                    .components(vec![])
                    .update();
                ctx.respond(slash).await
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
