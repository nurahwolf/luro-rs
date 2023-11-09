#![feature(return_position_impl_trait_in_trait)]
#![feature(async_fn_in_trait)]
#![feature(let_chains)]

use std::collections::{BTreeMap, HashMap};

use message::Message;
use story::Story;
use twilight_model::{
    application::interaction::Interaction,
    id::{marker::UserMarker, Id},
};

/// The primary owner user ID. Used for some defaults, as well as to say who owns the bot. This MUST  be set, even if a group of people own Luro, as its used as a fallback for when data is not tied to a specific user. For example, see [Story].
pub const PRIMARY_BOT_OWNER: Id<UserMarker> = Id::new(373524896187416576);
// Luro's primary owner(s)
pub const BOT_OWNERS: [Id<UserMarker>; 4] = [
    Id::new(1138489661187182692), // Zeron
    // Id::new(1146227925960638474), // Ferrona
    Id::new(138791390279630849), // Tzat
    Id::new(261308783546859520), // Aurora
    Id::new(373524896187416576), // Nurah
];
/// Luro's main accent colour
pub const ACCENT_COLOUR: u32 = 0xDABEEF;
/// Luro's DANGER colour
pub const COLOUR_DANGER: u32 = 0xD35F5F;
/// Transparent embed color (dark theme)
pub const COLOUR_TRANSPARENT: u32 = 0x2F3136;
/// Luro's SUCCESS colour
pub const COLOUR_SUCCESS: u32 = 0xA0D995;

/// The name used for Luro's webhooks
pub const WEBHOOK_NAME: &str = "LuroHook";

pub mod configuration;
pub mod functions;
// pub mod guild;
pub mod heck;
pub mod luro_database_driver;
pub mod message;
pub mod response;
// pub mod role;
pub mod story;
pub mod types;
pub mod user; // Migrated

#[cfg(feature = "builders")]
pub mod builders;

/// A simple wrapper around quotes. Primary key is the ID of the story.
pub type Quotes = BTreeMap<usize, Message>;

pub type Stories = BTreeMap<usize, Story>;

/// A [HashMap] containing an [Interaction], keyed by a [String]. Generally the message ID, but can be other markers too. This is primarily used for recalling interactions in the future
pub type CommandManager = HashMap<String, Interaction>;

// New
pub mod sync; // Types that are similar to multipe types, that can by synced to a single luro type.