use twilight_model::http::{attachment::Attachment, interaction::InteractionResponseType};

use crate::models::LuroResponse;

use super::LuroFramework;

impl LuroFramework {
    /// Using the data contained within this struct, respond to an interaction.
    pub async fn respond(&self, slash: &mut LuroResponse) -> anyhow::Result<()> {
        let client = self.interaction_client(slash);
        // Check to make sure fields are not too big, if they are send them as a file instead
        if let Some(content) = &mut slash.content {
            if content.len() > 2000 {
                slash.attachments = Some(vec![Attachment::from_bytes(
                    "fucking-huge-file.txt".to_owned(),
                    content.as_bytes().to_vec(),
                    1
                )]);
                content.truncate(1997);
                content.push_str("...");
            }
        }

        if let Some(embeds) = &mut slash.embeds {
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
                slash.attachments = Some(files);
            }
        }

        if slash.interaction_response_type == InteractionResponseType::DeferredChannelMessageWithSource {
            let mut response = client
                .update_response(&slash.interaction.token)
                .embeds(slash.embeds.as_deref())
                .components(slash.components.as_deref())
                .allowed_mentions(slash.allowed_mentions.as_ref());

            if let Some(content) = &slash.content && !content.is_empty() {
                    response = response.content(Some(content));
                }

            if let Some(attachments) = &slash.attachments {
                response = response.attachments(attachments)
            }

            response.await?;
        } else {
            self.interaction_client(slash)
                .create_response(slash.interaction.id, &slash.interaction.token, &slash.interaction_response())
                .await?;
        }

        Ok(())
    }
}
