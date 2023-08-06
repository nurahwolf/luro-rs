use std::mem;

use crate::models::LuroResponse;
use anyhow::anyhow;

use twilight_gateway::MessageSender;
use twilight_http::{client::InteractionClient, Client};

use twilight_model::{
    application::interaction::{modal::ModalInteractionData, Interaction, InteractionData},
    channel::message::{AllowedMentions, Component, Embed, MentionType, MessageFlags},
    http::{
        attachment::Attachment,
        interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType}
    },
    id::{marker::ApplicationMarker, Id}
};

impl LuroResponse {
    pub fn new(interaction: Interaction, shard: MessageSender) -> Self {
        Self {
            interaction_response_type: InteractionResponseType::ChannelMessageWithSource,
            allowed_mentions: Some(AllowedMentions {
                parse: vec![MentionType::Everyone, MentionType::Roles, MentionType::Users],
                replied_user: false,
                roles: Vec::new(),
                users: Vec::new()
            }),
            attachments: Default::default(),
            choices: Default::default(),
            components: Default::default(),
            content: Default::default(),
            custom_id: Default::default(),
            embeds: Default::default(),
            flags: Default::default(),
            title: Default::default(),
            tts: Default::default(),
            interaction,
            shard
        }
    }

    /// Parse incoming [`ModalSubmit`] interaction and return the inner data.
    ///
    /// This takes a mutable [`Interaction`] since the inner [`ModalInteractionData`]
    /// is replaced with [`None`] to avoid useless clones.
    ///
    /// [`ModalSubmit`]: twilight_model::application::interaction::InteractionType::ModalSubmit
    /// [`ModalInteractionData`]: twilight_model::application::interaction::modal::ModalInteractionData
    pub fn parse_modal_data(&self, interaction: &mut Interaction) -> anyhow::Result<ModalInteractionData> {
        match mem::take(&mut interaction.data) {
            Some(InteractionData::ModalSubmit(data)) => Ok(data),
            _ => Err(anyhow!("unable to parse modal data, received unknown data type"))
        }
    }

    /// Add an embed to the response. An error is returned if there are over 10 embeds already.
    pub fn embed(&mut self, embed: Embed) -> anyhow::Result<&mut Self> {
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
    pub fn title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        self
    }

    /// Set the custom_id of a response
    pub fn custom_id(&mut self, custom_id: impl Into<String>) -> &mut Self {
        self.custom_id = Some(custom_id.into());
        self
    }

    /// Set the content of a response
    pub fn content(&mut self, content: impl Into<String>) -> &mut Self {
        self.content = Some(content.into());
        self
    }

    /// Set the components of a response
    pub fn components(&mut self, components: Vec<Component>) -> &mut Self {
        self.components = Some(components);
        self
    }

    /// Set's the response type to be ephemeral
    pub fn ephemeral(&mut self) -> &mut Self {
        // TODO: Check to make sure we are responding to an interaction, otherwise this type cannot be used
        self.flags = Some(MessageFlags::EPHEMERAL);
        self
    }

    /// Set [InteractionResponseType::DeferredChannelMessageWithSource]
    pub fn set_deferred(&mut self) -> &mut Self {
        self.interaction_response_type = InteractionResponseType::DeferredChannelMessageWithSource;
        self
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
    pub async fn respond(mut self, twilight_client: &Client, slash: LuroResponse) -> anyhow::Result<()> {
        // Check to make sure fields are not too big, if they are send them as a file instead
        if let Some(content) = &mut self.content {
            if content.len() > 2000 {
                // Defer the message if it is not already
                // if self.interaction_response_type != InteractionResponseType::DeferredChannelMessageWithSource {
                //     self.deferred().await?;
                // }

                self.attachments = Some(vec![Attachment::from_bytes(
                    "fucking-huge-file.txt".to_owned(),
                    content.as_bytes().to_vec(),
                    1
                )]);
                content.truncate(1997);
                content.push_str("...");
            }
        }

        if let Some(embeds) = &mut self.embeds {
            let mut files_present = false;
            let mut file_id = 0;
            let mut files = vec![];
            let mut modified_embeds = vec![];

            for embed in embeds {
                if let Some(description) = &mut embed.description {
                    if description.len() > 4096 {
                        file_id += 1;

                        files.push(Attachment::from_bytes(
                            format!("Embed-{file_id}.txt"),
                            description.as_bytes().to_vec(),
                            file_id
                        ));

                        description.truncate(4093);
                        description.push_str("...");
                        files_present = true;
                    }
                }

                for field in &mut embed.fields {
                    if field.value.len() > 1000 {
                        file_id += 1;

                        files.push(Attachment::from_bytes(
                            format!("Field-{file_id}.txt"),
                            field.value.as_bytes().to_vec(),
                            file_id
                        ));

                        field.value.truncate(4093);
                        field.value.push_str("...");
                        files_present = true;
                    }
                }

                modified_embeds.push(embed.clone())
            }

            if files_present {
                self.attachments = Some(files);
            }
        }

        if self.interaction_response_type == InteractionResponseType::DeferredChannelMessageWithSource {
            let client = self.interaction_client(twilight_client, &slash.interaction.application_id);
            let mut response = client
                .update_response(&slash.interaction.token)
                .embeds(self.embeds.as_deref())
                .components(self.components.as_deref())
                .allowed_mentions(self.allowed_mentions.as_ref());

            if let Some(content) = &self.content && !content.is_empty() {
                        response = response.content(Some(content));
                    }

            if let Some(attachments) = &self.attachments {
                response = response.attachments(attachments)
            }

            response.await?;
        } else {
            self.interaction_client(twilight_client, &slash.interaction.application_id)
                .create_response(slash.interaction.id, &slash.interaction.token, &self.interaction_response())
                .await?;
        }

        Ok(())
    }

    /// Create an interaction client
    pub fn interaction_client<'a>(
        &'a self,
        twilight_client: &'a Client,
        application_id: &'a Id<ApplicationMarker>
    ) -> InteractionClient {
        twilight_client.interaction(*application_id)
    }
}
