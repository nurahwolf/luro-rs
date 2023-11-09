#[derive(Default)]
pub enum LuroCharacterFetishCategory {
    Favourite,
    Love,
    Like,
    #[default]
    Neutral,
    Dislike,
    Hate,
    Limit,
}

impl std::fmt::Display for LuroCharacterFetishCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LuroCharacterFetishCategory::Favourite => write!(f, "Favourite"),
            LuroCharacterFetishCategory::Love => write!(f, "Love"),
            LuroCharacterFetishCategory::Like => write!(f, "Like"),
            LuroCharacterFetishCategory::Neutral => write!(f, "Neutral"),
            LuroCharacterFetishCategory::Dislike => write!(f, "Dislike"),
            LuroCharacterFetishCategory::Hate => write!(f, "Hate"),
            LuroCharacterFetishCategory::Limit => write!(f, "Limit"),
        }
    }
}
