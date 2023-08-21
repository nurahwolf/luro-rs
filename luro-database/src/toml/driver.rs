use crate::toml::USERDATA_FILE_PATH;
use anyhow::anyhow;
use luro_model::{
    database::drivers::LuroDatabaseDriver,
    guild::LuroGuild,
    heck::{Heck, Hecks},
    message::LuroMessage,
    story::Story,
    user::{LuroUser, LuroUsers},
    CommandManager, Quotes, Stories, BOT_OWNERS
};
use std::{
    collections::{BTreeMap, HashMap},
    path::Path
};
use twilight_model::application::interaction::Interaction;

use super::{
    TomlDatabaseDriver, GUILDSETTINGS_FILE_PATH, INTERACTION_FILE_PATH, NSFW_HECK_FILE_PATH, NSFW_STORIES_FILE_PATH,
    QUOTES_FILE_PATH, SFW_HECK_FILE_PATH, SFW_STORIES_FILE_PATH
};

impl LuroDatabaseDriver for TomlDatabaseDriver {
    async fn add_guild(&self, id: u64, guild: &LuroGuild) -> anyhow::Result<()> {
        let path = format!("{0}/{1}/guild_settings.toml", GUILDSETTINGS_FILE_PATH, &id);
        Self::write(guild, Path::new(&path)).await
    }

    async fn add_heck(&self, heck: &Heck, nsfw: bool) -> anyhow::Result<()> {
        let path = match nsfw {
            true => Path::new(NSFW_HECK_FILE_PATH),
            false => Path::new(SFW_HECK_FILE_PATH)
        };
        let data = Self::get(path).await?;
        let mut data: Hecks = toml_deserializer(data)?;

        let total_hecks = data.len() + 1;
        data.insert(total_hecks, heck.clone());

        let data = toml_serializer(data);
        Self::write(data, path).await
    }

    async fn add_stories(&self, stories: &Stories, nsfw: bool) -> anyhow::Result<()> {
        let path = match nsfw {
            true => Path::new(NSFW_STORIES_FILE_PATH),
            false => Path::new(SFW_STORIES_FILE_PATH)
        };
        let data = Self::get(path).await?;
        let mut data: Stories = toml_deserializer(data)?;

        let mut total_stories = data.len() + 1;
        for story in stories.values() {
            data.insert(total_stories, story.clone());
            total_stories += 1;
        }

        let data = toml_serializer(data);
        Self::write(data, path).await
    }

    async fn add_story(&self, story: &Story, nsfw: bool) -> anyhow::Result<()> {
        let path = match nsfw {
            true => Path::new(NSFW_STORIES_FILE_PATH),
            false => Path::new(SFW_STORIES_FILE_PATH)
        };
        let data = Self::get(path).await?;
        let mut data: Stories = toml_deserializer(data)?;

        let total_stories = data.len() + 1;
        data.insert(total_stories, story.clone());

        let data = toml_serializer(data);
        Self::write(data, path).await
    }

    async fn get_guild(&self, id: u64) -> anyhow::Result<LuroGuild> {
        let path = format!("{0}/{1}/guild_settings.toml", GUILDSETTINGS_FILE_PATH, &id);
        Self::get(Path::new(&path)).await
    }

    async fn get_hecks(&self, nsfw: bool) -> anyhow::Result<Hecks> {
        let path = match nsfw {
            true => Path::new(NSFW_HECK_FILE_PATH),
            false => Path::new(SFW_HECK_FILE_PATH)
        };

        let data = Self::get(path).await?;
        let data: Hecks = toml_deserializer(data)?;
        Ok(data)
    }

    async fn get_heck(&self, id: &usize, nsfw: bool) -> anyhow::Result<luro_model::heck::Heck> {
        let path = match nsfw {
            true => Path::new(NSFW_HECK_FILE_PATH),
            false => Path::new(SFW_HECK_FILE_PATH)
        };

        let data = Self::get(path).await?;
        let data: Hecks = toml_deserializer(data)?;
        let data = match data.get(id) {
            Some(data) => Ok(data.clone()),
            None => Err(anyhow!("Heck with ID {id} not present!"))
        };
        data
    }

    async fn get_stories(&self, nsfw: bool) -> anyhow::Result<luro_model::Stories> {
        let path = match nsfw {
            true => Path::new(NSFW_STORIES_FILE_PATH),
            false => Path::new(SFW_STORIES_FILE_PATH)
        };

        let data = Self::get(path).await?;
        let data: Stories = toml_deserializer(data)?;
        Ok(data)
    }

    async fn get_story(&self, id: &usize, nsfw: bool) -> anyhow::Result<luro_model::story::Story> {
        let path = match nsfw {
            true => Path::new(NSFW_STORIES_FILE_PATH),
            false => Path::new(SFW_STORIES_FILE_PATH)
        };
        let data = Self::get(path).await?;
        let data: Stories = toml_deserializer(data)?;

        let story = match data.get(id) {
            Some(story) => Ok(story.clone()),
            None => Err(anyhow!("Story with ID {id} not present!"))
        };
        story
    }

    async fn get_user(&self, id: u64) -> anyhow::Result<LuroUser> {
        let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &id);
        Self::get(Path::new(&path)).await
    }

    /// Modify the guild settings and flush it to disk. This WILL overwrite all data locally!
    async fn update_guild(&self, id: u64, guild: &LuroGuild) -> anyhow::Result<()> {
        let path = format!("{0}/{1}/guild_settings.toml", GUILDSETTINGS_FILE_PATH, &id);
        Self::write(guild, Path::new(&path)).await
    }

    async fn modify_heck(&self, id: usize, heck: &Heck, nsfw: bool) -> anyhow::Result<()> {
        let path = match nsfw {
            true => Path::new(NSFW_HECK_FILE_PATH),
            false => Path::new(SFW_HECK_FILE_PATH)
        };
        let data = Self::get(path).await?;
        let mut data: Hecks = toml_deserializer(data)?;

        data.insert(id, heck.clone());

        let data = toml_serializer(data);
        Self::write(data, path).await
    }

    async fn modify_hecks(&self, modified_hecks: &Hecks, nsfw: bool) -> anyhow::Result<()> {
        let path = match nsfw {
            true => Path::new(NSFW_HECK_FILE_PATH),
            false => Path::new(SFW_HECK_FILE_PATH)
        };
        let data = Self::get(path).await?;
        let mut data: Hecks = toml_deserializer(data)?;

        for (heck_id, modified_heck) in modified_hecks.clone() {
            data.insert(heck_id, modified_heck);
        }

        let data = toml_serializer(data);
        Self::write(data, path).await
    }

    async fn modify_stories(&self, modified_stories: &luro_model::Stories, nsfw: bool) -> anyhow::Result<()> {
        let path = match nsfw {
            true => Path::new(NSFW_STORIES_FILE_PATH),
            false => Path::new(SFW_STORIES_FILE_PATH)
        };
        let data = Self::get(path).await?;
        let mut data: Stories = toml_deserializer(data)?;

        let mut total_stories = data.len() + 1;
        for story in modified_stories.values() {
            data.insert(total_stories, story.clone());
            total_stories += 1;
        }

        let data = toml_serializer(data);
        Self::write(data, path).await
    }

    async fn modify_story(&self, id: &usize, story: &luro_model::story::Story, nsfw: bool) -> anyhow::Result<()> {
        let path = match nsfw {
            true => Path::new(NSFW_STORIES_FILE_PATH),
            false => Path::new(SFW_STORIES_FILE_PATH)
        };
        let data = Self::get(path).await?;
        let mut data: Stories = toml_deserializer(data)?;

        data.insert(*id, story.clone());

        let data = toml_serializer(data);
        Self::write(data, path).await
    }

    async fn modify_user(&self, id: u64, user: &LuroUser) -> anyhow::Result<()> {
        let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &id);
        Self::write(user, Path::new(&path)).await
    }

    async fn remove_guild(&self, id: u64) -> anyhow::Result<()> {
        let path = format!("{0}/{1}/guild_settings.toml", GUILDSETTINGS_FILE_PATH, &id);
        Self::gdpr_delete(Path::new(&path)).await
    }

    async fn remove_heck(&self, id: usize, nsfw: bool) -> anyhow::Result<()> {
        let path = match nsfw {
            true => Path::new(NSFW_HECK_FILE_PATH),
            false => Path::new(SFW_HECK_FILE_PATH)
        };
        let data = Self::get(path).await?;
        let mut data: Hecks = toml_deserializer(data)?;

        data.remove(&id);

        let data = toml_serializer(data);
        Self::write(data, path).await
    }

    async fn remove_story(&self, id: usize, nsfw: bool) -> anyhow::Result<()> {
        let path = match nsfw {
            true => Path::new(NSFW_STORIES_FILE_PATH),
            false => Path::new(SFW_STORIES_FILE_PATH)
        };

        let data = Self::get(path).await?;
        let mut data: Stories = toml_deserializer(data)?;

        data.remove(&id);

        let data = toml_serializer(data);
        Self::write(data, path).await
    }

    async fn remove_user(&self, id: u64) -> anyhow::Result<()> {
        let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &id);
        Self::gdpr_delete(Path::new(&path)).await
    }

    async fn save_guild(&self, id: u64, guild: LuroGuild) -> anyhow::Result<()> {
        let path = format!("{0}/{1}/guild_settings.toml", GUILDSETTINGS_FILE_PATH, &id);
        Self::write(guild, Path::new(&path)).await
    }

    async fn save_hecks(&self, hecks: Hecks, nsfw: bool) -> anyhow::Result<()> {
        let path = match nsfw {
            true => Path::new(NSFW_HECK_FILE_PATH),
            false => Path::new(SFW_HECK_FILE_PATH)
        };

        let toml = toml_serializer(hecks);
        Self::write(toml, path).await
    }

    async fn save_stories(&self, stories: Stories, nsfw: bool) -> anyhow::Result<()> {
        let path = match nsfw {
            true => Path::new(NSFW_STORIES_FILE_PATH),
            false => Path::new(SFW_STORIES_FILE_PATH)
        };

        let toml = toml_serializer(stories);
        Self::write(toml, path).await
    }

    async fn save_story(&self, story: &luro_model::story::Story, nsfw: bool) -> anyhow::Result<()> {
        let path = match nsfw {
            true => Path::new(NSFW_STORIES_FILE_PATH),
            false => Path::new(SFW_STORIES_FILE_PATH)
        };

        let toml = Self::get(path).await?;
        let mut data: Stories = toml_deserializer(toml)?;

        let total_stories = data.len() + 1;
        data.insert(total_stories, story.clone());

        let toml = toml_serializer(data);
        Self::write(toml, path).await
    }

    async fn save_user(&self, id: u64, user: &LuroUser) -> anyhow::Result<()> {
        let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &id);
        Self::write(user, Path::new(&path)).await
    }

    async fn get_staff(&self) -> anyhow::Result<LuroUsers> {
        let mut staff = HashMap::new();
        for id in BOT_OWNERS {
            let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &id);
            let data: LuroUser = Self::get(Path::new(&path)).await?;
            staff.insert(id, data);
        }

        Ok(staff)
    }

    async fn save_interaction(&self, interaction: &Interaction, key: &str) -> anyhow::Result<()> {
        let mut data: CommandManager = Self::get(Path::new(INTERACTION_FILE_PATH)).await?;
        data.insert(key.to_string(), interaction.clone());
        Self::write(data, Path::new(Path::new(INTERACTION_FILE_PATH))).await
    }

    async fn get_interaction(&self, key: &str) -> anyhow::Result<twilight_model::application::interaction::Interaction> {
        let data: CommandManager = Self::get(Path::new(INTERACTION_FILE_PATH)).await?;
        let data = match data.get(&key.to_string()) {
            Some(data) => Ok(data.clone()),
            None => Err(anyhow!("Interaction with ID {key} not present!"))
        };
        data
    }

    async fn save_quote(&self, quote: LuroMessage, key: usize) -> anyhow::Result<()> {
        let toml = Self::get(Path::new(QUOTES_FILE_PATH)).await?;
        let mut data: Quotes = toml_deserializer(toml)?;
        data.insert(key, quote);
        let toml = toml_serializer(data);
        Self::write(toml, Path::new(Path::new(QUOTES_FILE_PATH))).await
    }

    async fn save_quotes(&self, quotes: Quotes) -> anyhow::Result<()> {
        let toml = toml_serializer(quotes);
        Self::write(toml, Path::new(Path::new(QUOTES_FILE_PATH))).await
    }

    async fn get_quote(&self, key: usize) -> anyhow::Result<LuroMessage> {
        let toml = Self::get(Path::new(QUOTES_FILE_PATH)).await?;
        let data: Quotes = toml_deserializer(toml)?;
        let data = match data.get(&key) {
            Some(data) => Ok(data.clone()),
            None => Err(anyhow!("Quote with ID {key} not present!"))
        };
        data
    }

    async fn get_quotes(&self) -> anyhow::Result<Quotes> {
        let toml = Self::get(Path::new(QUOTES_FILE_PATH)).await?;
        let data: Quotes = toml_deserializer(toml)?;
        Ok(data)
    }
}

// Serialise a BTreeMap, changing the key from usize to String
pub fn toml_serializer<T>(input: BTreeMap<usize, T>) -> BTreeMap<String, T> {
    input
        .into_iter()
        .map(|(str_key, value)| (str_key.to_string(), value))
        .collect::<BTreeMap<String, T>>()
}

// Deserialise a BTreeMap, changing the key from [String] to [usize]
pub fn toml_deserializer<T>(input: BTreeMap<String, T>) -> anyhow::Result<BTreeMap<usize, T>> {
    let original_len = input.len();
    let data = input
        .into_iter()
        .map(|(key, value)| {
            (
                match key.parse() {
                    Ok(usize_key) => usize_key,
                    Err(_) => todo!()
                },
                value
            )
        })
        .collect::<BTreeMap<usize, T>>();

    // multiple strings could parse to the same int, e.g "0" and "00"
    if data.len() < original_len {
        return Err(anyhow!("detected duplicate integer key"));
    }

    Ok(data)
}
