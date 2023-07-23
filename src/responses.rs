use anyhow::anyhow;
use twilight_model::{
    application::{command::CommandOptionChoice, interaction::Interaction},
    channel::message::{AllowedMentions, Component, Embed, MessageFlags},
    http::{
        attachment::Attachment,
        interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType}
    },
    id::{
        marker::{ApplicationMarker, InteractionMarker},
        Id
    }
};
use twilight_util::builder::embed::EmbedBuilder;

use crate::{models::LuroResponse, LuroContext};

pub mod ban;
pub mod bot_hierarchy;
pub mod bot_missing_permissions;
pub mod internal_error;
pub mod invalid_heck;
pub mod kick;
pub mod no_guild_settings;
pub mod no_interaction_channel;
pub mod not_guild;
pub mod not_member;
pub mod not_owner;
pub mod server_owner;
pub mod unable_to_get_guild;
pub mod unknown_command;
pub mod user_hierarchy;


/// Some nice stuff about formatting a response, ready to send via twilight's client
#[derive(Clone, Debug, PartialEq)]
pub struct LuroResponseV2 {
    // /// Luro's context, used for utility such as setting the embed accent colour and for sending our response near the end.
    // pub ctx: LuroContext,
    /// The command name or invoker of this interaction, for logging
    // TODO: Consider changing this to &str?
    pub command_name: String,
    /// ID of the interaction.
    pub id: Id<InteractionMarker>,
    /// ID of the associated application.
    pub application_id: Id<ApplicationMarker>,
    /// Interaction token of the command.
    pub token: String,
    /// The interaction response type for our response. Defaults to [`InteractionResponseType::ChannelMessageWithSource`].
    pub interaction_response_type: InteractionResponseType,
    /// Allowed mentions of the response.
    pub allowed_mentions: Option<AllowedMentions>,
    /// List of attachments on the response.
    pub attachments: Option<Vec<Attachment>>,
    /// List of autocomplete alternatives.
    ///
    /// Can only be used with
    /// [`InteractionResponseType::ApplicationCommandAutocompleteResult`].
    pub choices: Option<Vec<CommandOptionChoice>>,
    /// List of components on the response.
    pub components: Option<Vec<Component>>,
    /// Content of the response.
    pub content: Option<String>,
    /// For [`InteractionResponseType::Modal`], user defined identifier.
    pub custom_id: Option<String>,
    /// Embeds of the response.
    pub embeds: Option<Vec<Embed>>,
    /// Interaction response data flags.
    ///
    /// The supported flags are [`MessageFlags::SUPPRESS_EMBEDS`] and
    /// [`MessageFlags::EPHEMERAL`].
    pub flags: Option<MessageFlags>,
    /// For [`InteractionResponseType::Modal`], title of the modal.
    pub title: Option<String>,
    /// Whether the response is TTS.
    pub tts: Option<bool>
}

impl LuroResponseV2 {
    /// Create a new interaction response. Note that not setting anything else will not cause a response to be sent!
    /// This is set with some defaults:
    /// - AllowedMentions - All
    /// - InteractionResponseType - [`InteractionResponseType::ChannelMessageWithSource`]
    pub fn new(command_name: String, interaction: &Interaction) -> Self {
        // TODO: Set allowed_mentions to actually allow all
        Self {
            command_name,
            id: interaction.id,
            application_id: interaction.application_id,
            token: interaction.token.clone(),
            interaction_response_type: InteractionResponseType::ChannelMessageWithSource,
            allowed_mentions: None,
            attachments: None,
            choices: None,
            components: None,
            content: None,
            custom_id: None,
            embeds: None,
            flags: None,
            title: None,
            tts: None
        }
    }

    /// Add an embed to the response. An error is returned if there are over 10 embeds already.
    pub fn embed(mut self, embed: EmbedBuilder) -> anyhow::Result<Self> {
        if let Some(ref mut embeds) = self.embeds {
            // Check to make sure we have room
            if embeds.len() > 10 {
                return Err(anyhow!(
                    "There are already 10 embeds in this response, which is the limit that can be sent."
                ));
            }

            embeds.push(embed.build());
        } else {
            self.embeds = Some(vec![embed.build()])
        }
        Ok(self)
    }

    /// Add multiple embeds to the response. An error is returned if the total amount of embeds is over 10.
    /// The passed collection of embeds will be drained, this function does not clone.
    pub fn embeds(mut self, mut embeds: Vec<Embed>) -> anyhow::Result<Self> {
        if let Some(ref mut existing_embeds) = self.embeds {
            // Check to make sure we have room
            if existing_embeds.len() > 10 {
                return Err(anyhow!(
                    "There are already 10 embeds in this response, which is the limit that can be sent."
                ));
            }

            existing_embeds.append(&mut embeds);

            // Check again to make sure we have not gone over any size constraints
            if existing_embeds.len() > 10 {
                return Err(anyhow!(
                    "You just added embeds for a total of over 10 in this response, which is the limit that can be sent."
                ));
            }
        } else {
            self.embeds = Some(embeds)
        }
        Ok(self)
    }

    /// Set the content of a response
    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    /// Set's the response type to be ephemeral
    pub fn ephemeral(&mut self) -> &mut Self {
        // TODO: Check to make sure we are responding to an interaction, otherwise this type cannot be used
        self.flags = Some(MessageFlags::EPHEMERAL);
        self
    }

    /// Set's the response type to be sent as a response to a deferred message and acknowledge this interaction.
    pub async fn deferred(mut self, ctx: &LuroContext, ephemeral: bool) -> anyhow::Result<Self> {
        if ephemeral {
            self.ephemeral();
        };
        // TODO: Check to make sure we are responding to an interaction, otherwise this type cannot be used
        self.interaction_response_type = InteractionResponseType::DeferredChannelMessageWithSource;
        self.respond(ctx).await?;
        Ok(self)
    }

    /// Set the response to be a model
    pub fn model(&mut self) -> &mut Self {
        self.interaction_response_type = InteractionResponseType::Modal;
        self
    }

    /// Set the response to be an update response
    pub fn update(&mut self) -> &mut Self {
        self.interaction_response_type = InteractionResponseType::UpdateMessage;
        self
    }

    /// Set the response to be an update deferred response
    pub fn update_deferred(&mut self) -> &mut Self {
        self.interaction_response_type = InteractionResponseType::DeferredUpdateMessage;
        self
    }

    /// Sets the internal interaction_response. Called indirectly on sender functions.
    pub fn interaction_response(&self) -> InteractionResponse {
        InteractionResponse {
            kind: self.interaction_response_type,
            data: Some(InteractionResponseData {
                allowed_mentions: self.allowed_mentions.clone(),
                attachments: self.attachments.clone(),
                choices: self.choices.clone(),
                components: self.components.clone(),
                content: self.content.clone(),
                custom_id: self.custom_id.clone(),
                embeds: self.embeds.clone(),
                flags: self.flags,
                title: self.title.clone(),
                tts: self.tts
            })
        }
    }

    /// Using the data contained within this struct, respond to an interaction.
    pub async fn respond(&self, ctx: &LuroContext) -> anyhow::Result<()> {
        let client = ctx.twilight_client.interaction(self.application_id);
        client
            .create_response(self.id, &self.token, &self.interaction_response())
            .await?;

        Ok(())
    }

    /// A legacy return type that returns a [``] type, while old commands are migrated
    pub fn legacy_response(self, deferred: bool) -> crate::interactions::InteractionResponse {
        let response = self.interaction_response();
        crate::interactions::InteractionResponse::Raw {
            kind: response.kind,
            data: response.data,
            luro_response: LuroResponse {
                ephemeral: self.flags.is_some(),
                deferred
            }
        }
    }
}
