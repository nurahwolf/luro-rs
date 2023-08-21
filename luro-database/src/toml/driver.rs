use crate::toml::USERDATA_FILE_PATH;
use anyhow::anyhow;
use luro_model::{
    database::drivers::LuroDatabaseDriver,
    guild::LuroGuild,
    heck::{Heck, Hecks},
    message::LuroMessage,
    story::Story,
    user::LuroUser,
    CommandManager, Quotes, Stories
};
use std::path::Path;
use twilight_model::application::interaction::Interaction;

use super::{
    deserialize_quotes::deserialize_quotes, serialize_quotes::serialize_quotes, TomlDatabaseDriver, GUILDSETTINGS_FILE_PATH,
    INTERACTION_FILE_PATH, NSFW_HECK_FILE_PATH, NSFW_STORIES_FILE_PATH, QUOTES_FILE_PATH, SFW_HECK_FILE_PATH,
    SFW_STORIES_FILE_PATH
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

        let mut data: Hecks = Self::get(path).await?;
        let total_hecks = data.len() + 1;
        data.insert(total_hecks, heck.clone());
        Self::write(data, path).await
    }

    async fn add_stories(&self, stories: &Stories, nsfw: bool) -> anyhow::Result<()> {
        let path = match nsfw {
            true => Path::new(NSFW_STORIES_FILE_PATH),
            false => Path::new(SFW_STORIES_FILE_PATH)
        };

        let mut data: Stories = Self::get(path).await?;
        let mut total_stories = data.len() + 1;
        for story in stories.values() {
            data.insert(total_stories, story.clone());
            total_stories += 1;
        }
        Self::write(data, path).await
    }

    async fn add_story(&self, story: &Story, nsfw: bool) -> anyhow::Result<()> {
        let path = match nsfw {
            true => Path::new(NSFW_STORIES_FILE_PATH),
            false => Path::new(SFW_STORIES_FILE_PATH)
        };

        let mut data: Stories = Self::get(path).await?;
        let total_stories = data.len() + 1;
        data.insert(total_stories, story.clone());
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

        let hecks: Hecks = Self::get(path).await?;
        Ok(hecks)
    }

    async fn get_heck(&self, id: &usize, nsfw: bool) -> anyhow::Result<luro_model::heck::Heck> {
        let path = match nsfw {
            true => Path::new(NSFW_HECK_FILE_PATH),
            false => Path::new(SFW_HECK_FILE_PATH)
        };

        let data: Hecks = Self::get(path).await?;
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

        let data: Stories = Self::get(path).await?;
        Ok(data)
    }

    async fn get_story(&self, id: &usize, nsfw: bool) -> anyhow::Result<luro_model::story::Story> {
        let path = match nsfw {
            true => Path::new(NSFW_STORIES_FILE_PATH),
            false => Path::new(SFW_STORIES_FILE_PATH)
        };

        let data: Stories = Self::get(path).await?;
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

        let mut data: Hecks = Self::get(path).await?;
        data.insert(id, heck.clone());
        Self::write(data, path).await
    }

    async fn modify_hecks(&self, modified_hecks: &Hecks, nsfw: bool) -> anyhow::Result<()> {
        let path = match nsfw {
            true => Path::new(NSFW_HECK_FILE_PATH),
            false => Path::new(SFW_HECK_FILE_PATH)
        };

        let mut data: Hecks = Self::get(path).await?;
        for (heck_id, modified_heck) in modified_hecks.clone() {
            data.insert(heck_id, modified_heck);
        }
        Self::write(data, path).await
    }

    async fn modify_stories(&self, modified_stories: &luro_model::Stories, nsfw: bool) -> anyhow::Result<()> {
        let path = match nsfw {
            true => Path::new(NSFW_STORIES_FILE_PATH),
            false => Path::new(SFW_STORIES_FILE_PATH)
        };

        let mut data: Stories = Self::get(path).await?;
        let mut total_stories = data.len() + 1;
        for story in modified_stories.values() {
            data.insert(total_stories, story.clone());
            total_stories += 1;
        }
        Self::write(data, path).await
    }

    async fn modify_story(&self, id: &usize, story: &luro_model::story::Story, nsfw: bool) -> anyhow::Result<()> {
        let path = match nsfw {
            true => Path::new(NSFW_STORIES_FILE_PATH),
            false => Path::new(SFW_STORIES_FILE_PATH)
        };

        let mut data: Stories = Self::get(path).await?;
        data.insert(*id, story.clone());
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

        let mut data: Hecks = Self::get(path).await?;
        data.remove(&id);
        Self::write(data, path).await
    }

    async fn remove_story(&self, id: usize, nsfw: bool) -> anyhow::Result<()> {
        let path = match nsfw {
            true => Path::new(NSFW_STORIES_FILE_PATH),
            false => Path::new(SFW_STORIES_FILE_PATH)
        };

        let mut data: Stories = Self::get(path).await?;
        data.remove(&id);
        Self::write(data, path).await
    }

    async fn remove_user(&self, id: u64) -> anyhow::Result<()> {
        let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &id);
        Self::gdpr_delete(Path::new(&path)).await
    }

    async fn save_guild(&self, id: u64, guild: &LuroGuild) -> anyhow::Result<()> {
        let path = format!("{0}/{1}/guild_settings.toml", GUILDSETTINGS_FILE_PATH, &id);
        Self::write(guild, Path::new(&path)).await
    }

    async fn save_hecks(&self, hecks: &Hecks, nsfw: bool) -> anyhow::Result<()> {
        let path = match nsfw {
            true => Path::new(NSFW_HECK_FILE_PATH),
            false => Path::new(SFW_HECK_FILE_PATH)
        };
        Self::write(hecks, path).await
    }

    async fn save_stories(&self, stories: &luro_model::Stories, nsfw: bool) -> anyhow::Result<()> {
        let path = match nsfw {
            true => Path::new(NSFW_STORIES_FILE_PATH),
            false => Path::new(SFW_STORIES_FILE_PATH)
        };

        Self::write(stories, Path::new(path)).await
    }

    async fn save_story(&self, story: &luro_model::story::Story, nsfw: bool) -> anyhow::Result<()> {
        let path = match nsfw {
            true => Path::new(NSFW_STORIES_FILE_PATH),
            false => Path::new(SFW_STORIES_FILE_PATH)
        };

        let mut data: Stories = Self::get(path).await?;
        let total_stories = data.len() + 1;
        data.insert(total_stories, story.clone());
        Self::write(data, path).await
    }

    async fn save_user(&self, id: u64, user: &LuroUser) -> anyhow::Result<()> {
        let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &id);
        Self::write(user, Path::new(&path)).await
    }

    async fn get_staff(&self) -> anyhow::Result<luro_model::user::LuroUsers> {
        todo!()
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
        let mut data: Quotes = deserialize_quotes(toml)?;
        data.insert(key, quote);
        let toml = serialize_quotes(data);
        Self::write(toml, Path::new(Path::new(QUOTES_FILE_PATH))).await
    }

    async fn save_quotes(&self, quotes: Quotes) -> anyhow::Result<()> {
        let toml = serialize_quotes(quotes);
        Self::write(toml, Path::new(Path::new(QUOTES_FILE_PATH))).await
    }

    async fn get_quote(&self, key: usize) -> anyhow::Result<LuroMessage> {
        let toml = Self::get(Path::new(QUOTES_FILE_PATH)).await?;
        let data: Quotes = deserialize_quotes(toml)?;
        let data = match data.get(&key) {
            Some(data) => Ok(data.clone()),
            None => Err(anyhow!("Quote with ID {key} not present!"))
        };
        data
    }

    async fn get_quotes(&self) -> anyhow::Result<Quotes> {
        let toml = Self::get(Path::new(QUOTES_FILE_PATH)).await?;
        let data: Quotes = deserialize_quotes(toml)?;
        Ok(data)
    }
}
