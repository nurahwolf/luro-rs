use std::convert::TryInto;

use anyhow::Context;
use async_trait::async_trait;

use luro_model::story::Story;
use rand::Rng;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::message_component::MessageComponentInteractionData;
use twilight_model::application::interaction::modal::ModalInteractionData;
use twilight_model::channel::message::component::{ActionRow, Button, ButtonStyle, TextInput, TextInputStyle};
use twilight_model::channel::message::Component;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFooterBuilder};

use crate::COLOUR_DANGER;

#[cfg(not(feature = "toml-driver"))]
fn format_story_id(input: usize) -> usize {
    input
}

#[cfg(feature = "toml-driver")]
fn format_story_id(input: usize) -> String {
    input.to_string()
}
use crate::slash::Slash;
use crate::traits::luro_command::LuroCommand;
use crate::traits::luro_functions::LuroFunctions;
#[derive(CommandModel, CreateCommand)]
#[command(
    name = "story",
    desc = "Maybe you get a real story. Maybe you get a shitpost. Nobody knows."
)]
pub struct StoryCommand {
    /// If you want a specific story...
    id: Option<i64>,
    /// Set to true if you don't want the story to be in an embed
    plaintext: Option<bool>,
    /// Set to true to be given some details about the stories, like total amount
    info: Option<bool>,
    /// Set to true if you want a NSFW story in particular. Set to false for a SFW story.
    nsfw: Option<bool>,
    /// Set this to true if you want to add a story. All other options are ignored.
    add: Option<bool>
}

#[async_trait]
impl LuroCommand for StoryCommand {
    async fn handle_model(data: ModalInteractionData, ctx: Slash) -> anyhow::Result<()> {
        let nsfw = ctx.interaction.channel.clone().unwrap().nsfw.unwrap_or(false);
        let id = match nsfw {
            true => ctx.framework.database.nsfw_stories.len() + 1,
            false => ctx.framework.database.sfw_stories.len() + 1
        };
        let title = ctx.parse_modal_field_required(&data, "story-title")?.to_owned();
        let description = ctx.parse_modal_field_required(&data, "story-description")?.to_owned();

        ctx.framework
            .database
            .modify_story(
                id,
                &Story {
                    title,
                    description,
                    author: ctx.interaction.author().unwrap().id
                },
                nsfw
            )
            .await?;
        Ok(())
    }

    async fn run_command(self, mut ctx: Slash) -> anyhow::Result<()> {
        if let Some(add) = self.add && add {
            let components = vec![Component::ActionRow(ActionRow {
                components: vec![Component::TextInput(TextInput {
                    custom_id: "story-title".to_owned(),
                    label: "The title for your story".to_owned(),
                    max_length: None,
                    min_length: Some(4),
                    placeholder: Some("Little Red Riding Hood".to_owned()),
                    required: Some(true),
                    style: TextInputStyle::Short,
                    value: None
                })]
            }),
            Component::ActionRow(ActionRow { components: vec![Component::TextInput(TextInput {
                custom_id: "story-description".to_owned(),
                label: "Shitpost to your hearts content here!".to_owned(),
                max_length: None,
                min_length: Some(60),
                placeholder: Some("There was once a little girl...".to_owned()),
                required: Some(true),
                style: TextInputStyle::Paragraph,
                value: None
            })]
            })];
            return ctx.custom_id("story-add".to_owned())
                .title("Copy and paste your cursed thing below...".to_owned())
                .components(components)
                .model()
                .respond()
                .await
        }

        let mut embed = ctx.default_embed().await?;
        let channel_nsfw = ctx.interaction.channel.clone().unwrap().nsfw;
        let nsfw = if let Some(nsfw) = self.nsfw {
            match nsfw {
                false => false,
                true => {
                    if let Some(channel_nsfw) = channel_nsfw {
                        if !channel_nsfw {
                            return ctx.content("This is a SFW channel you dumb shit").ephemeral().respond().await;
                        }
                    }
                    true
                }
            }
        } else {
            ctx.interaction.channel.clone().unwrap().nsfw.unwrap_or(false)
        };

        let stories = ctx.framework.database.get_stories(nsfw).await?;

        let story_id = format_story_id(if let Some(story_id) = self.id {
            story_id.try_into().unwrap()
        } else {
            if stories.is_empty() {
                return ctx
                    .content("No stories have been added yet! Sorry...")
                    .ephemeral()
                    .respond()
                    .await;
            }
            rand::thread_rng().gen_range(0..stories.len()) + 1
        });

        let story = match stories.get(&story_id) {
            Some(story) => story,
            None => {
                return ctx
                    .internal_error_response("There is no story with that ID.".to_owned())
                    .await
            }
        };

        // Make sure we are in a size limit to send as plaintext, otherwise we are sending as an embed...
        if let Some(plaintext) = self.plaintext && plaintext && story.description.len() < 2000 {
            return ctx.content(&story.description).respond().await
        };

        embed = embed
            .title(&story.title)
            .description(&story.description)
            .footer(EmbedFooterBuilder::new(format!("Story ID: {story_id}")));

        if let Some(info) = self.info && info {
            embed = embed.footer(EmbedFooterBuilder::new(format!("Story ID: {story_id} - Total Number of Stories {}", stories.len())));

        }

        let button = button("story", "Delete this cursed thing");

        ctx.embed(embed.build())?.components(button).respond().await
    }

    async fn handle_component(_: Box<MessageComponentInteractionData>, mut ctx: Slash) -> anyhow::Result<()> {
        let embed = EmbedBuilder::new()
            .color(COLOUR_DANGER)
            .title("REDACTED")
            .description(format!(
                "There used to be a story here, but <@{}> found it too cursed for their eyes.",
                ctx.interaction.author_id().context("Expected interaction author")?
            ))
            .build();

        ctx.embed(embed)?.components(vec![]).update().respond().await
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
            style: ButtonStyle::Danger,
            url: None
        })])
    })])
}
