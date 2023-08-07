use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LuroMessageSource {
    MessageUpdate,
    MessageDelete,
    MessageCreate,
    None
}
