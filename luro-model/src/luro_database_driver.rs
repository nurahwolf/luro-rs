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

    fn add_item(driver: Self::Driver, item: &Self::Item) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
    fn add_items(driver: Self::Driver, items: &Self::Container) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
    fn get_item(id: &Self::Id, ctx: Self::Driver) -> impl std::future::Future<Output = anyhow::Result<Option<Self::Item>>> + Send;
    fn get_items(ids: Vec<&Self::Id>, ctx: Self::Driver) -> impl std::future::Future<Output = anyhow::Result<Self::Container>> + Send;
    fn modify_item(
        driver: Self::Driver,
        id: &Self::Id,
        item: &Self::Item,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<Self::Item>>> + Send;
    fn modify_items(
        driver: Self::Driver,
        items: &Self::Container,
    ) -> impl std::future::Future<Output = anyhow::Result<Self::Container>> + Send;
    fn remove_item(id: &Self::Id, ctx: Self::Driver) -> impl std::future::Future<Output = anyhow::Result<Option<Self::Item>>> + Send;
    fn remove_items(ids: Vec<&Self::Id>, ctx: Self::Driver) -> impl std::future::Future<Output = anyhow::Result<Self::Container>> + Send;
}
