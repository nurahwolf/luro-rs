use anyhow::Error;

use twilight_model::{
    application::interaction::Interaction,
    channel::message::{Component, Embed, MessageFlags},
    http::interaction::{
        InteractionResponse as HttpInteractionResponse, InteractionResponseData,
        InteractionResponseType,
    },
    id::{
        marker::{ApplicationMarker, InteractionMarker},
        Id,
    },
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
        components: Option<Vec<Component>>,
        ephemeral: bool,
    },
    /// Respond with text.
    Text {
        content: String,
        components: Option<Vec<Component>>,
        ephemeral: bool,
    },
    /// Respond with a modal.
    Modal {
        custom_id: String,
        title: String,
        components: Vec<Component>,
    },
    /// Respond with an ephemeral [`DeferredChannelMessageWithSource`] interaction type.
    ///
    /// [`DeferredChannelMessageWithSource`]: InteractionResponseType::DeferredChannelMessageWithSource
    EphemeralDeferredMessage,
    /// Respond with a raw [`HttpInteractionResponse`].
    Raw {
        kind: InteractionResponseType,
        data: Option<InteractionResponseData>,
    },
    /// AAAAA
    Update {
        content: Option<String>,
        embeds: Option<Vec<Embed>>,
        components: Option<Vec<Component>>,
        ephemeral: bool,
    },
}

/// Credentials used to respond to an interaction.
#[derive(Debug)]
pub struct InteractionResponder {
    /// ID of the interaction.
    pub id: Id<InteractionMarker>,
    /// ID of the associated application.
    pub application_id: Id<ApplicationMarker>,
    /// Token of the command.
    pub token: String,
}

impl InteractionResponder {
    /// Initialize a new [`InteractionResponder`] from an incoming interaction.
    pub fn from_interaction(interaction: &Interaction) -> Self {
        Self {
            id: interaction.id,
            application_id: interaction.application_id,
            token: interaction.token.clone(),
        }
    }

    /// Send a response to an interaction.
    pub async fn respond(
        &self,
        ctx: &LuroFramework,
        response: InteractionResponse,
    ) -> Result<(), Error> {
        let client = ctx.twilight_client.interaction(self.application_id);

        client
            .create_response(self.id, &self.token, &response.into_http())
            .await?;

        Ok(())
    }
}

impl InteractionResponse {
    /// Convert the response into a [`HttpInteractionResponse`].
    fn into_http(self) -> HttpInteractionResponse {
        let kind = match self {
            Self::Modal { .. } => InteractionResponseType::Modal,
            Self::EphemeralDeferredMessage => {
                InteractionResponseType::DeferredChannelMessageWithSource
            }
            Self::Raw { kind, .. } => kind,
            Self::Update { .. } => InteractionResponseType::UpdateMessage,
            _ => InteractionResponseType::ChannelMessageWithSource,
        };

        let data = match self {
            Self::Embed {
                embeds,
                ephemeral,
                components,
            } => {
                let mut response = InteractionResponseDataBuilder::new().embeds(embeds);

                if ephemeral {
                    response = response.flags(MessageFlags::EPHEMERAL);
                }

                if let Some(components) = components {
                    response = response.components(components);
                }

                Some(response.build())
            }
            Self::Text {
                content,
                components,
                ephemeral,
            } => {
                let mut response = InteractionResponseDataBuilder::new().content(content);

                if ephemeral {
                    response = response.flags(MessageFlags::EPHEMERAL);
                }

                if let Some(components) = components {
                    response = response.components(components);
                }

                Some(response.build())
            }
            Self::Modal {
                custom_id,
                title,
                components,
            } => Some(
                InteractionResponseDataBuilder::new()
                    .custom_id(custom_id)
                    .title(title)
                    .components(components)
                    .build(),
            ),
            Self::EphemeralDeferredMessage => Some(
                InteractionResponseDataBuilder::new()
                    .flags(MessageFlags::EPHEMERAL)
                    .build(),
            ),
            Self::Raw { data, .. } => data,
            Self::Update { content, embeds, components, ephemeral } => {
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

                Some(response.build())
            },
            
        };

        HttpInteractionResponse { kind, data }
    }
}
