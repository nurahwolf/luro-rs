#![feature(async_fn_in_trait)]

use std::sync::{Arc, RwLock};

use luro_model::{
    configuration::Configuration, database_driver::LuroDatabaseDriver, guild::LuroGuilds, heck::Hecks, user::LuroUsers,
    CommandManager, Quotes, Stories,
};
use serde::{Deserialize, Serialize};
mod database;

#[cfg(feature = "toml-driver")]
pub mod driver_toml;

/// Luro's database context. This itself just handles an abstraction for saving and loading data from whatever database it is using in the backend, depending on the feature selected.
///
/// NOTE: With the TOML driver, usize keys are serialised as strings!
#[derive(Debug)]
pub struct LuroDatabase<D: LuroDatabaseDriver> {
    pub command_data: RwLock<CommandManager>,
    pub count: RwLock<usize>,
    pub driver: D,
    pub guild_data: Box<RwLock<LuroGuilds>>,
    pub hecks: RwLock<HeckManager>,
    pub quotes: RwLock<Quotes>,
    pub staff: RwLock<LuroUsers>,
    pub stories: RwLock<StoryManager>,
    pub user_data: Box<RwLock<LuroUsers>>,
    pub config: Arc<Configuration<D>>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct HeckManager {
    pub nsfw: Hecks,
    pub sfw: Hecks,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct StoryManager {
    pub nsfw: Stories,
    pub sfw: Stories,
}

/// A trait to implementing on how to store them in the database
pub trait LuroDatabaseItem {
    /// The item to fetch
    type Item;
    /// A type that represents the ID of the item
    type Id;
    /// A type wrapping the item, for when fetching multiple
    type Container;
    /// Additional context needed to manipulate a type
    type Additional;

    async fn add_item(item: &Self::Item) -> anyhow::Result<()>;
    async fn add_items(items: &Self::Container) -> anyhow::Result<()>;
    async fn get_item(id: &Self::Id, ctx: Self::Additional) -> anyhow::Result<Self::Item>;
    async fn get_items(ids: Vec<&Self::Id>, ctx: Self::Additional) -> anyhow::Result<Self::Container>;
    async fn modify_item(id: &Self::Id, item: &Self::Item) -> anyhow::Result<Option<Self::Item>>;
    async fn modify_items(items: &Self::Container) -> anyhow::Result<Self::Container>;
    async fn remove_item(id: &Self::Id, ctx: Self::Additional) -> anyhow::Result<Option<Self::Item>>;
    async fn remove_items(ids: Vec<&Self::Id>, ctx: Self::Additional) -> anyhow::Result<Self::Container>;
}
