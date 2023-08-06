use std::{convert::TryInto, path::Path};

use async_trait::async_trait;

use rand::Rng;
use tracing::info;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::message_component::MessageComponentInteractionData;

use twilight_model::channel::message::component::{ActionRow, Button, ButtonStyle};
use twilight_model::channel::message::Component;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFooterBuilder};

use crate::models::LuroResponse;
use crate::{models::GlobalData, STORIES_FILE_PATH};
use crate::{LuroContext, COLOUR_DANGER};

use crate::traits::luro_command::LuroCommand;
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
    info: Option<bool>
}

#[async_trait]
impl LuroCommand for StoryCommand {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let mut is_empty = false;
        let new_stories = GlobalData::get_stories(Path::new(STORIES_FILE_PATH)).await?.stories;
        let stories;
        let story;
        let story_id;

        {
            if ctx.data_global.read().stories.is_empty() {
                is_empty = true;
            };
        }

        {
            let mut global_data = ctx.data_global.write();

            if is_empty {
                info!("Out of random stories to get, so reloading config...");
                global_data.stories = new_stories;
                stories = global_data.stories.clone();
            } else {
                stories = global_data.stories.clone();
            }

            story_id = if let Some(story_id) = self.id {
                story_id.try_into().unwrap()
            } else {
                rand::thread_rng().gen_range(0..stories.len())
            };

            story = stories.get(story_id);
        }

        // Error handle our story
        let story = match story {
            Some(story) => story,
            None => {
                return ctx
                    .internal_error_response("There is no story with that ID.".to_owned(), &mut slash)
                    .await
            }
        };

        // Make sure we are in a size limit to send as plaintext, otherwise we are sending as an embed...
        if let Some(plaintext) = self.plaintext && plaintext && story.description.len() < 2000 {
            slash.content(&story.description);
            return ctx.respond(&mut slash).await
        };

        let mut embed = ctx
            .default_embed(&slash.interaction.guild_id)
            .title(&story.title)
            .description(&story.description)
            .footer(EmbedFooterBuilder::new(format!("Story ID: {story_id}")));

        if let Some(info) = self.info && info {
            embed = embed.footer(EmbedFooterBuilder::new(format!("Story ID: {story_id} - Total Number of Stories {}", stories.len())));

        }

        let button = button("story", "Delete this cursed thing");

        slash.embed(embed.build())?.components(button);
        ctx.respond(&mut slash).await
    }

    async fn handle_component(
        _: Box<MessageComponentInteractionData>,
        ctx: &LuroContext,
        slash: &mut LuroResponse
    ) -> anyhow::Result<()> {
        let (_, slash_author) = ctx.get_interaction_author(slash)?;
        let embed = EmbedBuilder::new()
            .color(COLOUR_DANGER)
            .title("REDACTED")
            .description(format!(
                "There used to be a story here, but <@{}> found it too cursed for their eyes.",
                slash_author.user_id
            ))
            .build();

        slash.embed(embed)?.components(vec![]).update();
        ctx.respond(slash).await
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
