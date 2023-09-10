use anyhow::anyhow;
use async_trait::async_trait;
use luro_builder::embed::EmbedBuilder;
use luro_framework::command::{LuroCommandTrait, LuroCommandBuilder};
use luro_framework::context::parse_modal_field::parse_modal_field_required;
use luro_framework::responses::SimpleResponse;
use luro_framework::{Framework, InteractionCommand, InteractionComponent, InteractionModal, LuroInteraction};
use luro_model::story::Story;
use luro_model::COLOUR_DANGER;
use rand::Rng;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::channel::message::component::{ActionRow, Button, ButtonStyle, TextInput, TextInputStyle};
use twilight_model::channel::message::Component;
use twilight_model::http::interaction::InteractionResponseType;

use luro_model::database::drivers::LuroDatabaseDriver;
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
    /// Set to true if you want a NSFW story in particular. Set to false for a SFW story.
    nsfw: Option<bool>,
    /// Set this to true if you want to add a story. All other options are ignored.
    add: Option<bool>
}

impl<D: LuroDatabaseDriver + 'static> LuroCommandBuilder<D> for StoryCommand {}


#[async_trait]
impl LuroCommandTrait for StoryCommand {
    async fn handle_modal<D: LuroDatabaseDriver>(ctx: Framework<D>, interaction: InteractionModal) -> anyhow::Result<()> {
        let nsfw = interaction.channel.clone().unwrap().nsfw.unwrap_or(false);
        let stories = ctx.database.get_stories(nsfw).await?;
        let id = stories.len() + 1;
        let title = parse_modal_field_required(&interaction.data, "story-title")?.to_owned();
        let description = parse_modal_field_required(&interaction.data, "story-description")?.to_owned();

        ctx.database
            .save_story(
                id,
                Story {
                    title,
                    description,
                    author: interaction.author_id()
                },
                nsfw
            )
            .await?;
        Ok(())
    }

    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework<D>,
        interaction: InteractionCommand
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        if let Some(add) = data.add && add {
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
            return interaction.respond(&ctx, |response|
                response.add_components(components).custom_id("story-add").response_type(InteractionResponseType::Modal).title("Copy and paste your cursed thing below...")
            ).await
        }
        let channel_nsfw = interaction.channel.clone().unwrap().nsfw;
        let nsfw = if let Some(nsfw) = data.nsfw {
            match nsfw {
                false => false,
                true => {
                    if let Some(channel_nsfw) = channel_nsfw {
                        if !channel_nsfw {
                            return interaction
                                .respond(&ctx, |r| r.content("This is a SFW channel you dumb shit"))
                                .await;
                        }
                    }
                    true
                }
            }
        } else {
            interaction.channel.clone().unwrap().nsfw.unwrap_or(false)
        };

        let stories = ctx.database.get_stories(nsfw).await?;

        let story_id = if let Some(story_id) = data.id {
            story_id.try_into().unwrap()
        } else {
            if stories.is_empty() {
                return interaction
                    .respond(&ctx, |r| r.content("No stories of this type has been added!"))
                    .await;
            }
            rand::thread_rng().gen_range(0..stories.len()) + 1
        };

        let story = match stories.get(&story_id) {
            Some(story) => story,
            None => {
                return SimpleResponse::InternalError(&anyhow!("There is no story with ID {story_id}."))
                    .respond(&ctx, &interaction)
                    .await
            }
        };

        // Make sure we are in a size limit to send as plaintext, otherwise we are sending as an embed...
        if data.plaintext.unwrap_or_default() && story.description.len() < 2000 {
            return interaction.respond(&ctx, |r| r.content(story.description.clone())).await;
        };

        let accent_colour = interaction.accent_colour(&ctx).await;
        interaction
            .respond(&ctx, |r| {
                r.embed(|embed| {
                    embed
                        .title(&story.title)
                        .description(&story.description)
                        .footer(|f| f.text(format!("Story ID: {story_id} - Total Number of Stories {}", stories.len())))
                        .colour(accent_colour)
                })
                .add_components(button("story", "Delete this cursed thing"))
            })
            .await
    }

    async fn handle_component<D: LuroDatabaseDriver>(
        ctx: Framework<D>,
        interaction: InteractionComponent
    ) -> anyhow::Result<()> {
        let mut embed = EmbedBuilder::default();
        embed.colour(COLOUR_DANGER).title("REDACTED").description(format!(
            "There used to be a story here, but <@{}> found it too cursed for their eyes.",
            interaction.author_id()
        ));

        interaction
            .respond(&ctx, |response| response.add_embed(embed).components(|c| c).update())
            .await
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
