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
    #[allow(unreachable_code)]
    pub fn embed<F>(&mut self, embed: F) -> &mut Self
    where
        F: FnOnce(&mut EmbedBuilder) -> &mut EmbedBuilder
    {
        let mut e = EmbedBuilder::default();
        embed(&mut e);

        #[cfg(feature = "auto-trim")]
        return self.check_embed(vec![e.into()]);

        self
    }

    /// Add an embed without modifying the existing embeds, if present.
    ///
    /// NOTE: This WILL fail to send if more than 10 embeds are present!
    #[allow(unreachable_code)]
    pub fn add_embed(&mut self, embed: impl Into<Embed>) -> &mut Self {
        #[cfg(feature = "auto-trim")]
        return self.check_embed(vec![embed.into()]);

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
    #[allow(unreachable_code)]
    pub fn set_embeds(&mut self, embeds: Vec<Embed>) -> &mut Self {
        #[cfg(feature = "auto-trim")]
        return self.check_embed(embeds);

        self.embeds = Some(embeds);
        self
    }

    #[cfg(feature = "auto-trim")]
    fn check_embed(&mut self, embeds: Vec<Embed>) -> &mut Self {
        let mut file_id = 0;
        let mut files = vec![];
        let mut modified_embeds = vec![];

        for mut embed in embeds {
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
                }
            }
            modified_embeds.push(embed.clone())
        }

        if !files.is_empty() {
            self.attachments = Some(files);
        }

        match &mut self.embeds {
            Some(embeds) => embeds.append(&mut modified_embeds),
            None => self.embeds = Some(modified_embeds)
        }
        self
    }
}
