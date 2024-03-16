#[cfg(feature = "module-ai")]
mod ai;
#[cfg(feature = "module-interactions")]
mod interactions;
#[cfg(feature = "module-keywords")]
mod keyword;
#[cfg(feature = "module-prefix")]
mod prefix;

#[cfg(feature = "module-ai")]
pub use ai::ai_command_handler;
#[cfg(feature = "module-interactions")]
pub use interactions::{default_commands, interaction_handler};
#[cfg(feature = "module-keywords")]
pub use keyword::keyword_handler;
#[cfg(feature = "module-prefix")]
pub use prefix::prefix_handler;