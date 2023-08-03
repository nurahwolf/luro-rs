use anyhow::Context;
use regex::Regex;
use serde::de;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serializer;
use std::collections::BTreeMap;
use std::path::Path;
use twilight_model::id::marker::UserMarker;
use twilight_model::id::Id;

use crate::models::UserData;
use crate::LuroContext;
use crate::USERDATA_FILE_PATH;

use crate::traits::toml::LuroTOML;

use super::LuroMessage;

impl LuroTOML for UserData {}

impl UserData {
    /// This function just gets user settings and ensures it is in Luro's context.
    pub async fn write_user_settings(ctx: &LuroContext, user_id: &Id<UserMarker>) -> anyhow::Result<Self> {
        // Check to see if our data is present. if it is, return early
        {
            if let Some(settings) = ctx.user_data.get(user_id) {
                return Ok(settings.clone());
            }
        }

        // If we got this far, we know we need to load from disk
        let user_settings = Self::get(Path::new(&format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, user_id))).await?;

        // Now insert it into our context
        {
            ctx.user_data.insert(*user_id, user_settings.clone());
        }

        // Return the settings loaded from disk
        Ok(user_settings)
    }

    /// This function just gets user settings and ensures it is in Luro's context.
    pub async fn get_user_settings(ctx: &LuroContext, user_id: &Id<UserMarker>) -> anyhow::Result<Self> {
        // Check to see if our data is present. if it is, return early
        {
            if let Some(settings) = ctx.user_data.get(user_id) {
                return Ok(settings.clone());
            }
        }

        // If we got this far, we know we need to load from disk
        let user_settings = Self::get(Path::new(&format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, user_id))).await?;

        // Now insert it into our context
        {
            ctx.user_data.insert(*user_id, user_settings.clone());
        }

        // Return the settings loaded from disk
        Ok(user_settings)
    }

    /// Write new words
    pub async fn write_words(
        ctx: &LuroContext,
        new_words: &str,
        user_id: &Id<UserMarker>,
        message: &LuroMessage
    ) -> anyhow::Result<()> {
        // Make sure is valid
        let mut modified_user_data = UserData::get_user_settings(ctx, user_id)
            .await
            .context("Failed to get user data")?;

        // Add the raw message to the user's data
        modified_user_data.messages.insert(message.id, message.clone());

        if let Some(ref user) = ctx.twilight_cache.user(*user_id) {
            modified_user_data.accent_color = user.accent_color;
            modified_user_data.avatar = user.avatar;
            modified_user_data.banner = user.banner;
            modified_user_data.bot = user.bot;
            modified_user_data.discriminator = Some(user.discriminator().get());
            modified_user_data.email = user.email.clone();
            modified_user_data.flags = user.flags;
            modified_user_data.id = Some(user.id);
            modified_user_data.locale = user.locale.clone();
            modified_user_data.mfa_enabled = user.mfa_enabled;
            modified_user_data.name = Some(user.name.clone());
            modified_user_data.premium_type = user.premium_type;
            modified_user_data.public_flags = user.public_flags;
            modified_user_data.system = user.system;
            modified_user_data.verified = user.verified;
        }

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
            let mut user_data = ctx.user_data.get_mut(user_id).context("Expected user data to be present")?;
            *user_data = modified_user_data.clone()
        }

        // Write it to file
        modified_user_data
            .write(Path::new(&format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, user_id)))
            .await
    }
}

pub fn serialize_wordsize<S>(input: &BTreeMap<usize, usize>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    let data = input
        .iter()
        .map(|(str_key, value)| (str_key.to_string(), *value))
        .collect::<BTreeMap<String, usize>>();

    s.collect_map(data)
}

pub fn deserialize_wordsize<'de, D>(deserializer: D) -> Result<BTreeMap<usize, usize>, D::Error>
where
    D: Deserializer<'de>
{
    let str_map = BTreeMap::<String, usize>::deserialize(deserializer)?;
    let original_len = str_map.len();
    let data = {
        str_map
            .into_iter()
            .map(|(str_key, value)| match str_key.parse() {
                Ok(int_key) => Ok((int_key, value)),
                Err(_) => Err(de::Error::invalid_value(
                    de::Unexpected::Str(&str_key),
                    &"a non-negative integer"
                ))
            })
            .collect::<Result<BTreeMap<_, _>, _>>()?
    };
    // multiple strings could parse to the same int, e.g "0" and "00"
    if data.len() < original_len {
        return Err(de::Error::custom("detected duplicate integer key"));
    }
    Ok(data)
}
