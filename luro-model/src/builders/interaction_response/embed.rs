impl super::InteractionResponseBuilder {
    /// Create and append an embed. Multiple calls will add multiple embeds.
    ///
    /// NOTE: This WILL fail to send if more than 10 embeds are present!
    ///
    /// Refer to the documentation for [`EmbedBuilder`] for more
    /// information.
    pub fn embed<F>(&mut self, embed: F) -> &mut Self
    where
        F: FnOnce(&mut crate::builders::EmbedBuilder) -> &mut crate::builders::EmbedBuilder,
    {
        let mut e = crate::builders::EmbedBuilder::default();
        embed(&mut e);

        match &mut self.embeds {
            Some(embeds) => embeds.push(e.into()),
            None => self.embeds = Some(vec![e.into()]),
        }

        #[cfg(feature = "auto-trim")]
        return self.check_embed();

        #[cfg(not(feature = "auto-trim"))]
        self
    }

    /// Add an embed without modifying the existing embeds, if present.
    ///
    /// NOTE: This WILL fail to send if more than 10 embeds are present!
    pub fn add_embed(&mut self, embed: impl Into<twilight_model::channel::message::Embed>) -> &mut Self {
        match &mut self.embeds {
            Some(embeds) => embeds.push(embed.into()),
            None => self.embeds = Some(vec![embed.into()]),
        }

        #[cfg(feature = "auto-trim")]
        return self.check_embed();

        #[cfg(not(feature = "auto-trim"))]
        self
    }

    /// Explicitly set and overwrite all currently set embeds.
    /// Modify the nested embeds field for more advanced controls.
    ///
    /// NOTE: This WILL fail to send if more than 10 are present!
    pub fn set_embeds(&mut self, embeds: Vec<twilight_model::channel::message::Embed>) -> &mut Self {
        self.embeds = Some(embeds);

        #[cfg(feature = "auto-trim")]
        return self.check_embed();

        #[cfg(not(feature = "auto-trim"))]
        self
    }

    /// Explicitly set and overwrite all currently set embeds.
    /// Modify the nested embeds field for more advanced controls.
    pub fn set_embed(&mut self, embeds: impl Into<twilight_model::channel::message::Embed>) -> &mut Self {
        self.embeds = Some(vec![embeds.into()]);

        #[cfg(feature = "auto-trim")]
        return self.check_embed();

        #[cfg(not(feature = "auto-trim"))]
        self
    }

    #[cfg(feature = "auto-trim")]
    fn check_embed(&mut self) -> &mut Self {
        use crate::response::safe_truncate;

        if let Some(embeds) = &mut self.embeds {
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
                            file_id,
                        ));

                        safe_truncate(description, 4093);
                        description.push_str("...");
                    }
                }

                for field in &mut embed.fields {
                    if field.value.len() > 1000 {
                        file_id += 1;

                        files.push(Attachment::from_bytes(
                            format!("Field-{file_id}.txt"),
                            field.value.as_bytes().to_vec(),
                            file_id,
                        ));

                        safe_truncate(&mut field.value, 997);
                        field.value.push_str("...");
                    }
                }

                modified_embeds.push(embed.clone())
            }
            if !files.is_empty() {
                self.attachments(files.into_iter());
            }

            self.embeds = Some(modified_embeds)
        }
        self
    }
}
