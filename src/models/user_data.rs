use anyhow::Context;
use regex::Regex;
use std::path::Path;
use twilight_model::id::marker::UserMarker;
use twilight_model::id::Id;

use crate::models::UserData;
use crate::LuroContext;
use crate::USERDATA_FILE_PATH;

use super::toml::LuroTOML;

impl LuroTOML for UserData {}

impl UserData {
    /// This function just gets user settings and ensures it is in Luro's context.
    pub async fn write_user_settings(ctx: &LuroContext, user_id: &Id<UserMarker>) -> anyhow::Result<Self> {
        // Check to see if our data is present. if it is, return early
        {
            let user_data = ctx.user_data.read().clone();
            if let Some(settings) = user_data.get(user_id) {
                return Ok(settings.clone());
            }
        }

        // If we got this far, we know we need to load from disk
        let user_settings = Self::get(Path::new(&format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, user_id))).await?;

        // Now insert it into our context
        {
            ctx.user_data.write().insert(*user_id, user_settings.clone());
        }

        // Return the settings loaded from disk
        Ok(user_settings)
    }

    /// This function just gets user settings and ensures it is in Luro's context.
    pub async fn get_user_settings(ctx: &LuroContext, user_id: &Id<UserMarker>) -> anyhow::Result<Self> {
        // Check to see if our data is present. if it is, return early
        {
            let user_data = ctx.user_data.read().clone();
            if let Some(settings) = user_data.get(user_id) {
                return Ok(settings.clone());
            }
        }

        // If we got this far, we know we need to load from disk
        let user_settings = Self::get(Path::new(&format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, user_id))).await?;

        // Now insert it into our context
        {
            ctx.user_data.write().insert(*user_id, user_settings.clone());
        }

        // Return the settings loaded from disk
        Ok(user_settings)
    }

    /// Write new words
    pub async fn write_words(ctx: &LuroContext, new_words: &str, user_id: &Id<UserMarker>) -> anyhow::Result<()> {
        // Make sure is valid
        let mut modified_user_data = UserData::get_user_settings(ctx, user_id)
            .await
            .context("Failed to get user data")?;

        // First perform analysis
        let regex = Regex::new(r"\b[\w-]+\b").unwrap();
        for capture in regex.captures_iter(new_words) {
            let word = match capture.get(0) {
                Some(word) => word.as_str().to_ascii_lowercase(),
                None => "".to_owned()
            };
            let size = word.len();

            modified_user_data.wordcount += 1;
            modified_user_data.averagesize += size;
            *modified_user_data.words.entry(word).or_insert(0) += 1;
            *modified_user_data.wordsize.entry(size).or_insert(0) += 1;
        }

        {
            // Now write that to the user's context
            let mut user_data_db = ctx.user_data.write();
            let user_data = user_data_db.get_mut(user_id).context("Expected user data to be present")?;
            *user_data = modified_user_data.clone()
        }

        // Write it to file
        modified_user_data
            .write(Path::new(&format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, user_id)))
            .await
    }
}
