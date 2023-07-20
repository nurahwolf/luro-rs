use anyhow::Error;

use twilight_model::{
    application::interaction::Interaction,
    channel::message::{Component, Embed, MessageFlags},
    http::interaction::{InteractionResponse as HttpInteractionResponse, InteractionResponseData, InteractionResponseType},
    id::{
        marker::{ApplicationMarker, InteractionMarker},
        Id
    }
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::framework::LuroFramework;

/// Response to an interaction.
///
/// This enum contains types that can be used to respond to an interaction.
#[derive(Debug, Clone, PartialEq)]
pub enum InteractionResponse {
    /// Respond with an embed.
    Embed {
        embeds: Vec<Embed>,
        ephemeral: bool,
        deferred: bool
    },
    /// Respond with an embed and components.
    EmbedComponents {
        embeds: Vec<Embed>,
        components: Vec<Component>,
        ephemeral: bool,
        deferred: bool
    },
    /// Respond with content, embed and components.
    ContentEmbedComponents {
        content: String,
        embeds: Vec<Embed>,
        components: Vec<Component>,
        ephemeral: bool,
        deferred: bool
    },
    /// Respond with content.
    Content {
        content: String,
        ephemeral: bool,
        deferred: bool
    },
    /// Respond with content and embeds.
    ContentEmbed {
        content: String,
        embeds: Vec<Embed>,
        ephemeral: bool,
        deferred: bool
    },
    /// Respond with content and components.
    ContentComponents {
        content: String,
        components: Vec<Component>,
        ephemeral: bool,
        deferred: bool
    },
    /// Respond with a modal.
    Modal {
        custom_id: String,
        title: String,
        components: Vec<Component>
    },
    /// Component Update
    Update {
        content: Option<String>,
        embeds: Option<Vec<Embed>>,
        components: Option<Vec<Component>>,
        ephemeral: bool
    },
    /// Respond with a raw [`HttpInteractionResponse`].
    Raw {
        kind: InteractionResponseType,
        data: Option<InteractionResponseData>,
        deferred: bool,
        ephemeral: bool
    },
    /// Send a response to defer the interaction, so that the command does not time out.
    Defer {
        /// If the response should be seen by everyone else
        ephemeral: bool
    }
}

/// Credentials used to respond to an interaction.
#[derive(Debug)]
pub struct InteractionResponder {
    /// ID of the interaction.
    pub id: Id<InteractionMarker>,
    /// ID of the associated application.
    pub application_id: Id<ApplicationMarker>,
    /// Token of the command.
    pub token: String
}

impl InteractionResponder {
    /// Initialize a new [`InteractionResponder`] from an incoming interaction.
    pub fn from_interaction(interaction: &Interaction) -> Self {
        Self {
            id: interaction.id,
            application_id: interaction.application_id,
            token: interaction.token.clone()
        }
    }

    /// Create a new response
    pub async fn new_response(
        &self,
        ctx: &LuroFramework,
        response: &InteractionResponse,
        deferred: bool
    ) -> anyhow::Result<()> {
        let client = ctx.twilight_client.interaction(self.application_id);
        let response = response.clone().into_http();

        Ok(if deferred {
            // The inteaction was deferred, so respond as a follow up
            let interaction_response = response.data.unwrap_or(InteractionResponseData {
                allowed_mentions: None,
                attachments: None,
                choices: None,
                components: None,
                content: None,
                custom_id: None,
                embeds: None,
                flags: Some(MessageFlags::EPHEMERAL),
                title: None,
                tts: None
            });
            let mut followup = client.create_followup(&self.token);

            if let Some(content) = &interaction_response.content {
                followup = followup.content(content)?
            };

            if let Some(embeds) = &interaction_response.embeds {
                followup = followup.embeds(embeds)?;
            };

            if let Some(components) = &interaction_response.components {
                followup = followup.components(components)?;
            };

            if let Some(flags) = interaction_response.flags {
                followup = followup.flags(flags)
            }

            followup.await?;
        } else {
            // This is a new interaction, so create a response
            client.create_response(self.id, &self.token, &response).await?;
        })
    }

    /// Send a response to an interaction.
    pub async fn respond(&self, ctx: &LuroFramework, response: InteractionResponse) -> Result<(), Error> {
        match response {
            InteractionResponse::Embed { deferred, .. } => self.new_response(ctx, &response, deferred).await?,
            InteractionResponse::EmbedComponents { deferred, .. } => self.new_response(ctx, &response, deferred).await?,
            InteractionResponse::ContentEmbedComponents { deferred, .. } => self.new_response(ctx, &response, deferred).await?,
            InteractionResponse::Content { deferred, .. } => self.new_response(ctx, &response, deferred).await?,
            InteractionResponse::ContentComponents { deferred, .. } => self.new_response(ctx, &response, deferred).await?,
            InteractionResponse::ContentEmbed { deferred, .. } => self.new_response(ctx, &response, deferred).await?,
            InteractionResponse::Modal { .. } => self.new_response(ctx, &response, false).await?,
            InteractionResponse::Update { .. } => self.new_response(ctx, &response, false).await?,
            InteractionResponse::Defer { .. } => self.new_response(ctx, &response, false).await?,
            InteractionResponse::Raw { deferred, .. } => self.new_response(ctx, &response, deferred).await?
        };

        Ok(())
    }
}

impl InteractionResponse {
    /// Convert the response into a [`HttpInteractionResponse`].
    fn into_http(self) -> HttpInteractionResponse {
        let kind = match self {
            Self::Modal { .. } => InteractionResponseType::Modal,
            Self::Raw { kind, .. } => kind,
            Self::Update { .. } => InteractionResponseType::UpdateMessage,
            Self::Defer { .. } => InteractionResponseType::DeferredChannelMessageWithSource,
            _ => InteractionResponseType::ChannelMessageWithSource
        };

        let data = match self {
            Self::Embed { embeds, ephemeral, .. } => Some(interaction_response_builder(None, Some(embeds), None, ephemeral)),
            Self::EmbedComponents {
                embeds,
                components,
                ephemeral,
                ..
            } => Some(interaction_response_builder(None, Some(embeds), Some(components), ephemeral)),
            Self::ContentEmbedComponents {
                content,
                embeds,
                components,
                ephemeral,
                ..
            } => Some(interaction_response_builder(
                Some(content),
                Some(embeds),
                Some(components),
                ephemeral
            )),
            Self::Content { content, ephemeral, .. } => {
                Some(interaction_response_builder(Some(content), None, None, ephemeral))
            }
            Self::ContentComponents {
                content,
                components,
                ephemeral,
                ..
            } => Some(interaction_response_builder(Some(content), None, Some(components), ephemeral)),
            Self::Modal {
                custom_id,
                title,
                components
            } => Some(
                InteractionResponseDataBuilder::new()
                    .custom_id(custom_id)
                    .title(title)
                    .components(components)
                    .build()
            ),
            Self::Update {
                content,
                embeds,
                components,
                ephemeral
            } => Some(interaction_response_builder(content, embeds, components, ephemeral)),
            Self::Raw { data, .. } => data,
            Self::ContentEmbed {
                content,
                embeds,
                ephemeral,
                ..
            } => Some(interaction_response_builder(Some(content), Some(embeds), None, ephemeral)),
            Self::Defer { .. } => Some(InteractionResponseDataBuilder::new().flags(MessageFlags::EPHEMERAL).build())
        };

        HttpInteractionResponse { kind, data }
    }
}

fn interaction_response_builder(
    content: Option<String>,
    embeds: Option<Vec<Embed>>,
    components: Option<Vec<Component>>,
    ephemeral: bool
) -> InteractionResponseData {
    let mut response = InteractionResponseDataBuilder::new();

    if ephemeral {
        response = response.flags(MessageFlags::EPHEMERAL);
    }

    if let Some(content) = content {
        response = response.content(content);
    }

    if let Some(embeds) = embeds {
        response = response.embeds(embeds);
    }

    if let Some(components) = components {
        response = response.components(components);
    }

    response.build()
}
