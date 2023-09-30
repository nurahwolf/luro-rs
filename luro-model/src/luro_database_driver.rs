/// A trait to implementing on how to store them in the database
pub trait LuroDatabaseItem {
    /// The item to fetch
    type Item;
    /// A type that represents the ID of the item
    type Id;
    /// A type wrapping the item, for when fetching multiple
    type Container;
    /// Additional context needed to manipulate a type
    type Driver;

    async fn add_item(driver: Self::Driver, item: &Self::Item) -> anyhow::Result<()>;
    async fn add_items(driver: Self::Driver, items: &Self::Container) -> anyhow::Result<()>;
    async fn get_item(id: &Self::Id, ctx: Self::Driver) -> anyhow::Result<Option<Self::Item>>;
    async fn get_items(ids: Vec<&Self::Id>, ctx: Self::Driver) -> anyhow::Result<Self::Container>;
    async fn modify_item(driver: Self::Driver, id: &Self::Id, item: &Self::Item) -> anyhow::Result<Option<Self::Item>>;
    async fn modify_items(driver: Self::Driver, items: &Self::Container) -> anyhow::Result<Self::Container>;
    async fn remove_item(id: &Self::Id, ctx: Self::Driver) -> anyhow::Result<Option<Self::Item>>;
    async fn remove_items(ids: Vec<&Self::Id>, ctx: Self::Driver) -> anyhow::Result<Self::Container>;
}
