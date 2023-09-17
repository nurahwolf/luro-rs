use std::{collections::BTreeMap, path::Path};

use twilight_model::id::Id;

use crate::{luro_database_driver::LuroDatabaseItem, user::LuroUser};

use super::TomlDatabaseDriver;

const USERDATA_FILE_PATH: &str = "data/user";

impl LuroDatabaseItem for LuroUser {
    type Item = LuroUser;
    type Id = u64;
    type Container = BTreeMap<Self::Id, Self::Item>;
    type Additional = ();

    async fn add_item(item: &Self::Item) -> anyhow::Result<()> {
        let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &item.id);
        TomlDatabaseDriver::write(item, Path::new(&path)).await?;
        Ok(())
    }

    async fn add_items(items: &Self::Container) -> anyhow::Result<()> {
        for item in items.values() {
            let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &item.id);
            TomlDatabaseDriver::write(item, Path::new(&path)).await?;
        }
        Ok(())
    }

    async fn get_item(id: &Self::Id, _ctx: Self::Additional) -> anyhow::Result<Self::Item> {
        let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &id);
        TomlDatabaseDriver::get(Path::new(&path), LuroUser::new(Id::new(*id))).await
    }

    async fn get_items(ids: Vec<&Self::Id>, _ctx: Self::Additional) -> anyhow::Result<Self::Container> {
        let mut items = BTreeMap::new();
        for id in ids {
            let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &id);
            items.insert(
                *id,
                TomlDatabaseDriver::get(Path::new(&path), LuroUser::new(Id::new(*id))).await?,
            );
        }
        Ok(items)
    }

    async fn modify_item(id: &Self::Id, item: &Self::Item) -> anyhow::Result<Option<Self::Item>> {
        let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &id);
        Ok(Some(TomlDatabaseDriver::write(item, Path::new(&path)).await.cloned()?))
    }

    async fn modify_items(items: &Self::Container) -> anyhow::Result<Self::Container> {
        let mut old_items = BTreeMap::new();
        for (id, item) in items {
            let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &item.id);
            old_items.insert(
                *id,
                TomlDatabaseDriver::get(Path::new(&path), LuroUser::new(Id::new(*id))).await?,
            );
            TomlDatabaseDriver::write(item, Path::new(&path)).await?;
        }
        Ok(old_items)
    }

    async fn remove_item(id: &Self::Id, _ctx: Self::Additional) -> anyhow::Result<Option<Self::Item>> {
        let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &id);
        let old_item = TomlDatabaseDriver::get(Path::new(&path), LuroUser::new(Id::new(*id))).await?;
        TomlDatabaseDriver::gdpr_delete(Path::new(&path)).await?;
        Ok(Some(old_item))
    }

    async fn remove_items(ids: Vec<&Self::Id>, _ctx: Self::Additional) -> anyhow::Result<Self::Container> {
        let mut old_items = BTreeMap::new();
        for id in ids {
            let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &id);
            old_items.insert(
                *id,
                TomlDatabaseDriver::get(Path::new(&path), LuroUser::new(Id::new(*id))).await?,
            );
            TomlDatabaseDriver::gdpr_delete(Path::new(&path)).await?;
        }
        Ok(old_items)
    }
}
