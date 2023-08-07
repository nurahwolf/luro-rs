use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum UserActionType {
    Ban,
    Kick,
    Warn,
    PrivilegeEscalation
}
