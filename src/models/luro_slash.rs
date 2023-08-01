use std::str::FromStr;

use crate::{
    commands::base64::{Base64Decode, Base64Encode},
    models::{GuildSetting, LuroSlash},
    ACCENT_COLOUR
};
use anyhow::anyhow;
use tracing::{debug, error, info, warn};
use twilight_gateway::MessageSender;
use twilight_http::{client::InteractionClient, Response};
use twilight_model::{
    application::interaction::{Interaction, InteractionData, InteractionType},
    channel::{
        message::{AllowedMentions, Component, Embed, MentionType, MessageFlags},
        Channel, Message
    },
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
    user::User
};
use twilight_util::builder::embed::EmbedBuilder;

use crate::traits::luro_command::LuroCommand;
use crate::{
    commands::{boop::BoopCommand, heck::add::HeckAddCommand},
    LuroContext
};

use super::CustomId;

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

    // Handle an interaction
    pub async fn handle(self) -> anyhow::Result<()> {
        let response = match self.interaction.kind {
            InteractionType::ApplicationCommand => self.clone().handle_command().await,
            InteractionType::MessageComponent => self.clone().handle_component().await,
            InteractionType::ModalSubmit => self.clone().handle_modal().await,
            other => {
                warn!("received unexpected {} interaction", other.kind());
                Ok(())
            }
        };

        if let Err(why) = response {
            error!(error = ?why, "error while processing interaction");
            // Attempt to send an error response
            if let Err(send_fail) = self.internal_error_response(why.to_string()).await {
                error!(error = ?send_fail, "Failed to respond to the interaction with an error response");
            };
        };

        Ok(())
    }

    /// Handle incoming component interaction
    pub async fn handle_component(self) -> anyhow::Result<()> {
        let data = match self.interaction.data {
            Some(InteractionData::MessageComponent(ref data)) => data.clone(),
            _ => return Err(anyhow!("expected message component data"))
        };

        info!(
            "Received component interaction - {} - {}",
            self.author()?.name,
            data.custom_id
        );

        match &*data.custom_id {
            "boop" => BoopCommand::handle_button(Default::default(), self, data).await,
            "decode" => Base64Decode::handle_button(Default::default(), self, data).await,
            "encode" => Base64Encode::handle_button(Default::default(), self, data).await,
            "heck-setting" => HeckAddCommand::handle_component(Default::default(), self, data).await,
            name => {
                warn!(name = name, "received unknown component");
                self.unknown_command_response().await
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
                let embed = self
                    .default_embed()
                    .await?
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
    pub fn custom_id(mut self, custom_id: impl Into<String>) -> Self {
        self.custom_id = Some(custom_id.into());
        self
    }

    /// Set the content of a response
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
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

    /// Create an interaction client
    pub fn interaction_client(&self) -> InteractionClient {
        self.luro.twilight_client.interaction(self.interaction.application_id)
    }

    /// Set's the response type to be sent as a response to a deferred message and acknowledge this interaction.
    pub async fn deferred(&mut self) -> anyhow::Result<&mut Self> {
        // TODO: Check to make sure we are responding to an interaction, otherwise this type cannot be used
        self.interaction_response_type = InteractionResponseType::DeferredChannelMessageWithSource;
        self.interaction_client()
            .create_response(self.interaction.id, &self.interaction.token, &self.interaction_response())
            .await?;
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
        if self.interaction_response_type == InteractionResponseType::DeferredChannelMessageWithSource {
            let client = self.interaction_client();
            let mut response = client
                .update_response(&self.interaction.token)
                .embeds(self.embeds.as_deref())?
                .components(self.components.as_deref())?
                .allowed_mentions(self.allowed_mentions.as_ref());

            if let Some(content) = &self.content && !content.is_empty() {
                response = response.content(Some(content))?;
            }

            if let Some(attachments) = &self.attachments {
                response = response.attachments(attachments)?
            }

            response.await?;
        } else {
            self.interaction_client()
                .create_response(self.interaction.id, &self.interaction.token, &self.interaction_response())
                .await?;
        }

        Ok(())
    }

    /// Send a message, useful if you do not want to consume the interaction.
    pub async fn send_message(&self) -> anyhow::Result<Response<Message>> {
        //TODO: Change this to not unwrap and error handle
        let mut message = self
            .luro
            .twilight_client
            .create_message(self.interaction.channel.as_ref().unwrap().id);

        if let Some(embeds) = &self.embeds {
            message = message.embeds(embeds)?
        }

        if let Some(content) = &self.content {
            message = message.content(content)?
        }

        if let Some(components) = &self.components {
            message = message.components(components)?
        }

        if let Some(flags) = &self.flags {
            message = message.flags(*flags)
        }

        if let Some(interaction_message) = &self.interaction.message {
            message = message.reply(interaction_message.id)
        }

        Ok(message.await?)
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
        self.interaction
            .channel
            .clone()
            .ok_or_else(|| anyhow!("Unable to get the channel this interaction was ran in"))
    }

    /// Create a default embed which has the guild's accent colour if available, otherwise falls back to Luro's accent colour
    pub async fn default_embed(&self) -> anyhow::Result<EmbedBuilder> {
        Ok(EmbedBuilder::new().color(self.accent_colour().await?))
    }

    /// Attempts to get the guild's accent colour, else falls back to getting the hardcoded accent colour
    pub async fn accent_colour(&self) -> anyhow::Result<u32> {
        if let Some(guild_id) = &self.interaction.guild_id {
            let guild_settings = GuildSetting::get_guild_settings(&self.luro, guild_id).await?;

            // Check to see if a custom colour is defined
            if let Some(custom_accent_colour) = guild_settings.accent_colour_custom {
                return Ok(custom_accent_colour);
            };

            if guild_settings.accent_colour != 0 {
                return Ok(ACCENT_COLOUR);
            }
        };

        Ok(ACCENT_COLOUR)
    }
}
