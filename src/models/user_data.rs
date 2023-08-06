use anyhow::Context;
use dashmap::mapref::one::RefMut;
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
    /// Internal function for converting a user_id into a path
    fn path(user_id: &Id<UserMarker>) -> String {
        format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, user_id)
    }

    /// This function just gets user settings and ensures it is in Luro's context.
    pub async fn get_user_settings<'a>(ctx: &'a LuroContext, user_id: &Id<UserMarker>) -> anyhow::Result<Self> {
        match ctx.user_data.get(user_id) {
            Some(user_data) => Ok(user_data.clone()),
            None => {
                let user_settings = Self::get(Path::new(&Self::path(user_id))).await?;
                {
                    ctx.user_data.insert(*user_id, user_settings.clone());
                }
                Ok(ctx.user_data.get(user_id).context("Expected to find user_data")?.clone())
            }
        }
    }

    /// This function gets user settings and ensures it is in Luro's context, returning a context that can be modified.
    pub async fn modify_user_settings<'a>(
        ctx: &'a LuroContext,
        user_id: &Id<UserMarker>
    ) -> anyhow::Result<RefMut<'a, Id<UserMarker>, UserData>> {
        match ctx.user_data.get_mut(user_id) {
            Some(user_data) => Ok(user_data),
            None => {
                let user_settings = Self::get(Path::new(&Self::path(user_id))).await?;
                {
                    ctx.user_data.insert(*user_id, user_settings.clone());
                }
                Ok(ctx.user_data.get_mut(user_id).context("Expected to find user_data")?)
            }
        }
    }

    /// Write user data. This is a shorthand around [Self::write] which allows not needing to specify a path
    pub async fn write_user_data(&self, user_id: &Id<UserMarker>) -> anyhow::Result<()> {
        Self::write(self, Path::new(&Self::path(user_id))).await
    }

    /// Write new words
    pub async fn write_words(
        ctx: &LuroContext,
        new_words: &str,
        user_id: &Id<UserMarker>,
        message: &LuroMessage
    ) -> anyhow::Result<()> {
        let mut modified_user_data = UserData::modify_user_settings(ctx, user_id)
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

        modified_user_data.write_user_data(user_id).await
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
