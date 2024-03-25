#[cfg(feature = "builders")]
pub mod builders;
#[cfg(feature = "database-twilight")]
pub mod database;
mod models;
#[cfg(feature = "responses")]
pub mod response;

// This shortens the amount of nesting on models
pub use models::*;

// Colours
/// A 'Danger' colour, used for distructive icons like bans, or dangerous interactions.
pub const COLOUR_DANGER: u32 = 0xD35F5F;
/// A default colour. Uses [COLOUR_TRANSPARENT] by default.
pub const COLOUR_DEFAULT: u32 = COLOUR_TRANSPARENT;
/// A 'Error' colour, used for errors. Defaults to [COLOUR_DANGER]
pub const COLOUR_ERROR: u32 = COLOUR_DANGER;
/// A colour used for 'transpancy' on the default discord dark theme
pub const COLOUR_TRANSPARENT: u32 = 0x2F3136;
/// A 'Success' colour, used as a marker when a success should be explicily called out like changing settings.
pub const COLOUR_SUCCESS: u32 = 0xA0D995;

// Luro Settings
/// Luro's primary owner(s)
pub const BOT_OWNERS: [twilight_model::id::Id<twilight_model::id::marker::UserMarker>; 4] = [
    twilight_model::id::Id::new(1138489661187182692), // Zeron
    // twilight_model::id::Id::new(1146227925960638474), // Ferrona
    twilight_model::id::Id::new(138791390279630849), // Tzat
    twilight_model::id::Id::new(261308783546859520), // Aurora
    twilight_model::id::Id::new(373524896187416576), // Nurah
];

pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;
