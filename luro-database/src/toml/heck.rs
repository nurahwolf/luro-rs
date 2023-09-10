use std::{collections::BTreeMap, path::Path};

use anyhow::Context;
use luro_model::heck::{Heck, Hecks};

use crate::LuroDatabaseItem;

use super::{toml_deserializer, toml_serializer, TomlDatabaseDriver, NSFW_HECK_FILE_PATH, SFW_HECK_FILE_PATH};

impl LuroDatabaseItem for Heck {
    type Item = Heck;
    type Id = usize;
    type Container = Hecks;
    type Additional = bool;

    async fn add_item(item: &Self::Item) -> anyhow::Result<()> {
        // Get a NSFW or SFW heck
        let path = match item.nsfw {
            true => Path::new(NSFW_HECK_FILE_PATH),
            false => Path::new(SFW_HECK_FILE_PATH)
        };

        // Fetch data from disk, converting from toml specific data
        let mut data = toml_deserializer(TomlDatabaseDriver::get(path, Default::default()).await?)?;

        data.insert(data.len() + 1, item.clone());

        TomlDatabaseDriver::write(toml_serializer(data), path).await?;

        Ok(())
    }

    async fn add_items(items: &Self::Container) -> anyhow::Result<()> {
        let sfw_path = Path::new(SFW_HECK_FILE_PATH);
        let nsfw_path = Path::new(NSFW_HECK_FILE_PATH);

        let mut sfw_hecks = toml_deserializer(TomlDatabaseDriver::get(sfw_path, Default::default()).await?)?;
        let mut nsfw_hecks = toml_deserializer(TomlDatabaseDriver::get(nsfw_path, Default::default()).await?)?;

        for heck in items.values() {
            match heck.nsfw {
                true => nsfw_hecks.insert(nsfw_hecks.len() + 1, heck.clone()),
                false => sfw_hecks.insert(nsfw_hecks.len() + 1, heck.clone())
            };
        }

        TomlDatabaseDriver::write(toml_serializer(sfw_hecks), sfw_path).await?;
        TomlDatabaseDriver::write(toml_serializer(nsfw_hecks), nsfw_path).await?;

        Ok(())
    }

    async fn get_item(id: &Self::Id, ctx: Self::Additional) -> anyhow::Result<Self::Item> {
        let path = match ctx {
            true => Path::new(NSFW_HECK_FILE_PATH),
            false => Path::new(SFW_HECK_FILE_PATH)
        };

        let data = TomlDatabaseDriver::get(path, Default::default()).await?;
        let data: Hecks = toml_deserializer(data)?;
        data.get(id).context("Passed ID is not present").cloned()
    }

    async fn get_items(ids: Vec<&Self::Id>, ctx: Self::Additional) -> anyhow::Result<Self::Container> {
        let path = match ctx {
            true => Path::new(NSFW_HECK_FILE_PATH),
            false => Path::new(SFW_HECK_FILE_PATH)
        };

        let hecks = TomlDatabaseDriver::get(path, Default::default()).await?;
        let hecks: Hecks = toml_deserializer(hecks)?;

        // If the user passes an empty vector of IDs, return all of them
        if ids.is_empty() {
            return Ok(hecks);
        }

        // Otherwise return what the user has specified
        let mut items = BTreeMap::new();
        for id in ids {
            if let Some(heck) = hecks.get(id) {
                items.insert(*id, heck.clone());
            }
        }
        Ok(items)
    }

    async fn modify_item(id: &Self::Id, item: &Self::Item) -> anyhow::Result<Option<Self::Item>> {
        // Get a NSFW or SFW heck
        let path = match item.nsfw {
            true => Path::new(NSFW_HECK_FILE_PATH),
            false => Path::new(SFW_HECK_FILE_PATH)
        };

        // Fetch data from disk, converting from toml specific data
        let mut data = toml_deserializer(TomlDatabaseDriver::get(path, Default::default()).await?)?;
        let old_item = data.insert(*id, item.clone());
        TomlDatabaseDriver::write(toml_serializer(data), path).await?;

        Ok(old_item)
    }

    async fn modify_items(items: &Self::Container) -> anyhow::Result<Self::Container> {
        let sfw_path = Path::new(SFW_HECK_FILE_PATH);
        let nsfw_path = Path::new(NSFW_HECK_FILE_PATH);
        let mut old_items = BTreeMap::new();
        let mut sfw_hecks = toml_deserializer(TomlDatabaseDriver::get(sfw_path, Default::default()).await?)?;
        let mut nsfw_hecks = toml_deserializer(TomlDatabaseDriver::get(nsfw_path, Default::default()).await?)?;

        for (id, heck) in items {
            match heck.nsfw {
                true => {
                    if let Some(data) = nsfw_hecks.insert(*id, heck.clone()) {
                        old_items.insert(*id, data);
                    }
                }
                false => {
                    if let Some(data) = sfw_hecks.insert(*id, heck.clone()) {
                        old_items.insert(*id, data);
                    }
                }
            };
        }

        TomlDatabaseDriver::write(toml_serializer(sfw_hecks), sfw_path).await?;
        TomlDatabaseDriver::write(toml_serializer(nsfw_hecks), nsfw_path).await?;

        Ok(old_items)
    }

    async fn remove_item(id: &Self::Id, ctx: Self::Additional) -> anyhow::Result<Option<Self::Item>> {
        let path = match ctx {
            true => Path::new(NSFW_HECK_FILE_PATH),
            false => Path::new(SFW_HECK_FILE_PATH)
        };
        let data = TomlDatabaseDriver::get(path, Default::default()).await?;
        let mut data: Hecks = toml_deserializer(data)?;

        let old_data = data.remove(id);

        let data = toml_serializer(data);
        TomlDatabaseDriver::write(data, path).await?;
        Ok(old_data)
    }

    async fn remove_items(ids: Vec<&Self::Id>, ctx: Self::Additional) -> anyhow::Result<Self::Container> {
        let mut old_items = BTreeMap::new();
        let path = match ctx {
            true => Path::new(NSFW_HECK_FILE_PATH),
            false => Path::new(SFW_HECK_FILE_PATH)
        };
        let data = TomlDatabaseDriver::get(path, Default::default()).await?;
        let mut data: Hecks = toml_deserializer(data)?;

        for id in ids {
            if let Some(data) = data.remove(id) {
                old_items.insert(*id, data);
            }
        }

        let data = toml_serializer(data);
        TomlDatabaseDriver::write(data, path).await?;

        Ok(old_items)
    }
}
