#![feature(async_fn_in_trait)]
#![feature(let_chains)]

pub mod constants;
pub mod functions;
pub mod guild_permissions;
pub mod guild_setting;
pub mod heck;
pub mod luro_database;
pub mod luro_database_driver;
pub mod luro_log_channel;
pub mod luro_message;
pub mod luro_message_source;
pub mod luro_permissions;
pub mod luro_user;
pub mod member_roles;
pub mod quote;
pub mod role_ordering;
pub mod slash_user;
pub mod story;
pub mod types;
pub mod user_actions;
pub mod user_actions_type;
pub mod user_marriages;

// Dice rolling functionality
#[cfg(feature = "dice-roller")]
pub mod dice_roll;
#[cfg(feature = "dice-roller")]
pub mod filter_modifier;
#[cfg(feature = "dice-roller")]
pub mod roll_ast;
#[cfg(feature = "dice-roller")]
pub mod roll_options;
#[cfg(feature = "dice-roller")]
pub mod roll_parser;
#[cfg(feature = "dice-roller")]
pub mod roll_result;
#[cfg(feature = "dice-roller")]
pub mod roll_value;
