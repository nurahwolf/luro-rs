/// A list of assignable fetishes. Used for matching with other users
#[derive(Clone, Debug, Default, serde::Deserialize, PartialEq, serde::Serialize, Ord, PartialOrd, Eq)]
pub enum FetishList {
    #[default]
    Custom,
}

impl std::fmt::Display for FetishList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Custom => "Custom",
            }
        )
    }
}
