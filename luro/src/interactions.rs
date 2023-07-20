use anyhow::Error;

use twilight_http::{response::marker::EmptyBody, Response};
use twilight_model::{
    application::interaction::Interaction,
    channel::{
        message::{Component, Embed, MessageFlags},
        Message
    },
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
    Embed { embeds: Vec<Embed>, ephemeral: bool },
    /// Respond with an embed and components.
    EmbedComponents {
        embeds: Vec<Embed>,
        components: Vec<Component>,
        ephemeral: bool
    },
    /// Respond with content, embed and components.
    ContentEmbedComponents {
        content: String,
        embeds: Vec<Embed>,
        components: Vec<Component>,
        ephemeral: bool
    },
    /// Respond with content.
    Content { content: String, ephemeral: bool },
    /// Respond with content and embeds.
    ContentEmbed {
        content: String,
        embeds: Vec<Embed>,
        ephemeral: bool
    },
    /// Respond with content and components.
    ContentComponents {
        content: String,
        components: Vec<Component>,
        ephemeral: bool
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
        followup: bool,
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

    /// Respond with a followup
    pub async fn followup_response(
        &self,
        ctx: &LuroFramework,
        content: Option<String>,
        embeds: Option<Vec<Embed>>,
        components: Option<Vec<Component>>,
        ephemeral: bool
    ) -> Result<Response<Message>, Error> {
        let client = ctx.twilight_client.interaction(self.application_id);
        let mut followup = client.create_followup(&self.token);

        if let Some(content) = &content {
            followup = followup.content(content)?
        };

        if let Some(embeds) = &embeds {
            followup = followup.embeds(embeds)?;
        };

        if let Some(components) = &components {
            followup = followup.components(components)?;
        };

        if ephemeral {
            followup = followup.flags(MessageFlags::EPHEMERAL)
        }

        Ok(followup.await?)
    }

    /// Create a new response
    pub async fn new_response(
        &self,
        ctx: &LuroFramework,
        response: &InteractionResponse
    ) -> Result<Response<EmptyBody>, Error> {
        let client = ctx.twilight_client.interaction(self.application_id);
        Ok(client
            .create_response(self.id, &self.token, &response.clone().into_http())
            .await?)
    }

    /// Send a response to an interaction.
    pub async fn respond(&self, ctx: &LuroFramework, response: InteractionResponse) -> Result<(), Error> {
        match response {
            InteractionResponse::Embed { embeds, ephemeral } => self
                .followup_response(ctx, None, Some(embeds), None, ephemeral)
                .await?
                .status(),
            InteractionResponse::EmbedComponents {
                embeds,
                components,
                ephemeral
            } => self
                .followup_response(ctx, None, Some(embeds), Some(components), ephemeral)
                .await?
                .status(),
            InteractionResponse::ContentEmbedComponents {
                content,
                embeds,
                components,
                ephemeral
            } => self
                .followup_response(ctx, Some(content), Some(embeds), Some(components), ephemeral)
                .await?
                .status(),
            InteractionResponse::Content { content, ephemeral } => self
                .followup_response(ctx, Some(content), None, None, ephemeral)
                .await?
                .status(),
            InteractionResponse::ContentComponents {
                content,
                components,
                ephemeral
            } => self
                .followup_response(ctx, Some(content), None, Some(components), ephemeral)
                .await?
                .status(),
            InteractionResponse::Modal {
                custom_id: _,
                title: _,
                components: _
            } => self.new_response(ctx, &response).await?.status(),
            InteractionResponse::Raw {
                kind: _,
                data: _,
                followup,
                ephemeral
            } => {
                if followup {
                    let followup = response
                        .clone()
                        .into_http()
                        .data
                        .ok_or_else(|| Error::msg("Expected InteractionResponseData for followup interaction type"))?;

                    self.followup_response(ctx, followup.content, followup.embeds, followup.components, ephemeral)
                        .await?
                        .status()
                } else {
                    self.new_response(ctx, &response).await?.status()
                }
            }
            InteractionResponse::Update {
                content: _,
                embeds: _,
                components: _,
                ephemeral: _
            } => self.new_response(ctx, &response).await?.status(),
            InteractionResponse::ContentEmbed {
                content,
                embeds,
                ephemeral
            } => self
                .followup_response(ctx, Some(content), Some(embeds), None, ephemeral)
                .await?
                .status(),
            InteractionResponse::Defer { .. } => self.new_response(ctx, &response).await?.status()
            // InteractionResponse::Update {
            //     content,
            //     embeds,
            //     components,
            //     ephemeral,
            // } => self.followup_response(ctx, content, embeds, components, ephemeral).await?.status(),
            // _ => self.new_response(ctx, response).await?.status()
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
            Self::Embed { embeds, ephemeral } => Some(interaction_response_builder(None, Some(embeds), None, ephemeral)),
            Self::EmbedComponents {
                embeds,
                components,
                ephemeral
            } => Some(interaction_response_builder(None, Some(embeds), Some(components), ephemeral)),
            Self::ContentEmbedComponents {
                content,
                embeds,
                components,
                ephemeral
            } => Some(interaction_response_builder(
                Some(content),
                Some(embeds),
                Some(components),
                ephemeral
            )),
            Self::Content { content, ephemeral } => Some(interaction_response_builder(Some(content), None, None, ephemeral)),
            Self::ContentComponents {
                content,
                components,
                ephemeral
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
                ephemeral
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
