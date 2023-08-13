use twilight_model::{channel::message::Embed, http::attachment::Attachment};

use crate::embed::EmbedBuilder;

use super::LuroResponse;

impl LuroResponse {
    /// Create and append an embed. Multiple calls will add multiple embeds.
    ///
    /// NOTE: This WILL fail to send if more than 10 embeds are present!
    ///
    /// Refer to the documentation for [`EmbedBuilder`] for more
    /// information.
    pub fn embed<F>(&mut self, embed: F) -> &mut Self
    where
        F: FnOnce(&mut EmbedBuilder) -> &mut EmbedBuilder
    {
        let mut e = EmbedBuilder::default();
        embed(&mut e);

        let mut files_present = false;
        let mut file_id = 0;
        let mut files = vec![];

        #[cfg(feature = "auto-trim")]
        if let Some(description) = &mut e.0.description {
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

        #[cfg(feature = "auto-trim")]
        for field in &mut e.0.fields {
            if field.value.len() > 1000 {
                file_id += 1;

                files.push(Attachment::from_bytes(
                    format!("Field-{file_id}.txt"),
                    field.value.as_bytes().to_vec(),
                    file_id
                ));

                field.value.truncate(997);
                field.value.push_str("...");
                files_present = true;
            }
        }

        #[cfg(feature = "auto-trim")]
        if files_present {
            match &mut self.attachments {
                Some(attachments) => attachments.append(&mut files),
                None => self.attachments = Some(files)
            }
        }

        match &mut self.embeds {
            Some(embeds) => embeds.push(e.into()),
            None => self.embeds = Some(vec![e.into()])
        }

        self
    }

    /// Add an embed without modifying the existing embeds, if present.
    ///
    /// NOTE: This WILL fail to send if more than 10 embeds are present!
    pub fn add_embed(&mut self, embed: impl Into<Embed>) -> &mut Self {
        match &mut self.embeds {
            Some(embeds) => embeds.push(embed.into()),
            None => self.embeds = Some(vec![embed.into()])
        }
        self
    }

    /// Explicitly set and overwrite all currently set embeds.
    /// Modify the nested embeds field for more advanced controls.
    ///
    /// NOTE: This WILL fail to send if more than 10 are present!
    pub fn set_embeds(&mut self, embeds: Vec<Embed>) -> &mut Self {
        let mut files_present = false;
        let mut file_id = 0;
        let mut files = vec![];
        let mut modified_embeds = vec![];

        #[cfg(feature = "auto-trim")]
        for mut embed in embeds.clone() {
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

                    field.value.truncate(997);
                    field.value.push_str("...");
                    files_present = true;
                }
            }
            modified_embeds.push(embed.clone())
        }

        if files_present {
            self.embeds = Some(modified_embeds);
            self.attachments = Some(files);
        } else {
            self.embeds = Some(embeds);
        }

        self
    }
}
