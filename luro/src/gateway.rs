mod shard_runner;

pub type GatewayArc = Arc<Gateway>;
pub type GatewayResult = Result<(), GatewayError>;
use std::sync::Arc;

use crate::models::luro::GatewayError;
pub use crate::models::luro::Luro as Gateway;
pub use shard_runner::shard_runner;
