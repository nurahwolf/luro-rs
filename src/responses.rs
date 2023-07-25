use std::str::FromStr;

use crate::{commands::LuroCommand, functions::default_embed};
use anyhow::anyhow;
use tracing::{debug, error, warn};
use twilight_gateway::MessageSender;
use twilight_model::{
    application::{
        command::CommandOptionChoice,
        interaction::{Interaction, InteractionData, InteractionType}
    },
    channel::{
        message::{AllowedMentions, Component, Embed, MentionType, MessageFlags},
        Channel
    },
    http::{
        attachment::Attachment,
        interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType}
    },
    user::User
};
use twilight_util::builder::embed::EmbedBuilder;

use crate::{
    commands::{boop::BoopCommand, heck::add::HeckAddCommand},
    functions::CustomId,
    models::LuroResponse,
    LuroContext
};

pub mod ban;
mod bot_hierarchy;
mod bot_missing_permissions;
mod internal_error;
mod invalid_heck;
pub mod kick;
mod no_guild_settings;
mod no_interaction_channel;
mod not_guild;
mod not_member;
mod not_owner;
mod nsfw_in_sfw;
mod server_owner;
mod unable_to_get_guild;
mod unknown_command;
mod user_hierarchy;

/// Some nice stuff about formatting a response, ready to send via twilight's client
#[derive(Clone, Debug)]
pub struct LuroSlash {
    // /// Luro's context, used for utility such as setting the embed accent colour and for sending our response near the end.
    pub luro: LuroContext,
    /// Interaction we are handling
    pub interaction: Interaction,
    pub shard: MessageSender,
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

impl LuroSlash {
    /// Create a new interaction response. Note that not setting anything else will not cause a response to be sent!
    /// This is set with some defaults:
    /// - AllowedMentions - All
    /// - InteractionResponseType - [`InteractionResponseType::ChannelMessageWithSource`]
    pub fn new(ctx: LuroContext, interaction: Interaction, shard: MessageSender) -> Self {
        debug!(id = ?interaction.id, "Processing {} interaction", interaction.kind.kind());

        // TODO: Set allowed_mentions to actually allow all
        Self {
            luro: ctx,
            interaction,
            shard,
            interaction_response_type: InteractionResponseType::ChannelMessageWithSource,
            allowed_mentions: Some(AllowedMentions {
                parse: vec![MentionType::Everyone, MentionType::Roles, MentionType::Users],
                replied_user: false,
                roles: Vec::new(),
                users: Vec::new()
            }),
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

    /// Create a default embed (which has an accent colour defined) by using the data contained within the interaction.
    pub fn default_embed(&self) -> EmbedBuilder {
        default_embed(&self.luro, &self.interaction.guild_id)
    }

    // Handle an interaction
    pub async fn handle(self) -> anyhow::Result<()> {
        let response = match self.interaction.kind {
            InteractionType::ApplicationCommand => self.handle_command().await,
            InteractionType::MessageComponent => self.handle_component().await,
            InteractionType::ModalSubmit => self.handle_modal().await,
            other => {
                warn!("received unexpected {} interaction", other.kind());
                Ok(())
            }
        };

        if let Err(why) = response {
            error!(error = ?why, "error while processing interaction");
        };

        Ok(())
    }

    /// Handle incoming component interaction
    pub async fn handle_component(self) -> anyhow::Result<()> {
        let custom_id = match &self.interaction.data {
            Some(InteractionData::MessageComponent(data)) => CustomId::from_str(&data.custom_id)?,
            _ => return Err(anyhow!("expected message component data"))
        };

        match &*custom_id.name {
            "boop" => BoopCommand::handle_button(Default::default(), self).await,
            "heck-setting" => HeckAddCommand::handle_button(Default::default(), self).await,
            name => {
                warn!(name = name, "received unknown component");

                // TODO: Make this a response type.
                let embed = default_embed(&self.luro, &self.interaction.guild_id)
                    .title("IT'S FUCKED")
                    .description("Will finish this at some point");
                self.embeds(vec![embed.build()])?.respond().await
            }
        }
    }

    /// Handle incoming modal interaction
    pub async fn handle_modal(self) -> anyhow::Result<()> {
        let custom_id = match self.interaction.data {
            Some(InteractionData::ModalSubmit(ref data)) => CustomId::from_str(&data.custom_id)?,
            _ => return Err(anyhow!("expected modal submit data"))
        };

        match &*custom_id.name {
            "heck-add" => HeckAddCommand::handle_model(Default::default(), self).await,
            name => {
                warn!(name = name, "received unknown component");

                // TODO: Make this a response type.
                let embed = default_embed(&self.luro, &self.interaction.guild_id)
                    .title("IT'S FUCKED")
                    .description("Will finish this at some point");
                self.embeds(vec![embed.build()])?.respond().await
            }
        }
    }

    /// Add an embed to the response. An error is returned if there are over 10 embeds already.
    pub fn embed(mut self, embed: Embed) -> anyhow::Result<Self> {
        if let Some(ref mut embeds) = self.embeds {
            // Check to make sure we have room
            if embeds.len() > 10 {
                return Err(anyhow!(
                    "There are already 10 embeds in this response, which is the limit that can be sent."
                ));
            }

            embeds.push(embed);
        } else {
            self.embeds = Some(vec![embed])
        }
        Ok(self)
    }

    /// Add multiple embeds to the response. An error is returned if the total amount of embeds is over 10.
    /// NOTE: This CLEARS whatever is set to self.embeds, so if you want to keep them, make sure to clone it first
    pub fn embeds(mut self, embeds: Vec<Embed>) -> anyhow::Result<Self> {
        if embeds.len() > 10 {
            return Err(anyhow!(
                "There are already 10 embeds in this response, which is the limit that can be sent."
            ));
        }

        self.embeds = Some(embeds);
        Ok(self)
    }

    /// Set the title of a response
    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// Set the custom_id of a response
    pub fn custom_id(mut self, custom_id: String) -> Self {
        self.custom_id = Some(custom_id);
        self
    }

    /// Set the content of a response
    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    /// Set the components of a response
    pub fn components(mut self, components: Vec<Component>) -> Self {
        self.components = Some(components);
        self
    }

    /// Set's the response type to be ephemeral
    pub fn ephemeral(&mut self) -> &mut Self {
        // TODO: Check to make sure we are responding to an interaction, otherwise this type cannot be used
        self.flags = Some(MessageFlags::EPHEMERAL);
        self
    }

    /// Set's the response type to be sent as a response to a deferred message and acknowledge this interaction.
    pub async fn deferred(mut self) -> anyhow::Result<Self> {
        // TODO: Check to make sure we are responding to an interaction, otherwise this type cannot be used
        self.interaction_response_type = InteractionResponseType::DeferredChannelMessageWithSource;
        self.respond().await?;
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
    pub async fn respond(&self) -> anyhow::Result<()> {
        let client = self.luro.twilight_client.interaction(self.interaction.application_id);
        client
            .create_response(self.interaction.id, &self.interaction.token, &self.interaction_response())
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

    /// Get the interaction author.
    pub fn author(&self) -> anyhow::Result<User> {
        Ok(match &self.interaction.member {
            Some(member) => member
                .user
                .clone()
                .ok_or_else(|| anyhow!("Unable to find the user that executed this command"))?,
            None => self
                .interaction
                .user
                .clone()
                .ok_or_else(|| anyhow!("Unable to find the user that executed this command"))?
        })
    }

    /// Get the interaction channel.
    pub fn channel(&self) -> anyhow::Result<Channel> {
        self
            .interaction
            .channel
            .clone()
            .ok_or_else(|| anyhow!("Unable to get the channel this interaction was ran in"))
    }
}