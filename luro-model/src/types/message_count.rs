use twilight_model::id::{marker::UserMarker, Id};
#[derive(Debug, PartialEq, Eq)]
pub struct MessageCount {
    pub author_id: Option<Id<UserMarker>>,
    pub total_messages: i64,
    pub total_unique_words: i64,
    pub total_words: i64,
}

impl PartialOrd for MessageCount {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MessageCount {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total_unique_words.cmp(&other.total_unique_words).reverse()
    }
}
