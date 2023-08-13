//! These constants are user facing and used throughout Luro. They are intended to be updated by the end user.

use twilight_model::id::{marker::UserMarker, Id};

/// The primary owner user ID. Used for some defaults, as well as to say who owns the bot. This MUST  be set, even if a group of people own Luro, as its used as a fallback for when data is not tied to a specific user. For example, see [Story].
pub const PRIMARY_BOT_OWNER: Id<UserMarker> = Id::new(373524896187416576);
// Luro's primary owner(s)
pub const BOT_OWNERS: [Id<UserMarker>; 2] = [Id::new(373524896187416576), Id::new(138791390279630849)];
/// Luro's main accent colour
pub const ACCENT_COLOUR: u32 = 0xDABEEF;
/// Luro's DANGER colour
pub const COLOUR_DANGER: u32 = 0xD35F5F;
/// Transparent embed color (dark theme)
pub const COLOUR_TRANSPARENT: u32 = 0x2F3136;
/// Luro's SUCCESS colour
pub const COLOUR_SUCCESS: u32 = 0xA0D995;