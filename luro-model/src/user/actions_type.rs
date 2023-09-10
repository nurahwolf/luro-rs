use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum UserActionType {
    Ban,
    Kick,
    Warn,
    PrivilegeEscalation,
    #[default]
    None,
}
