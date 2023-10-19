mod count_marriages;
mod delete_marriage;
mod get_marriage;
mod get_marriages;
mod update_marriage;

#[derive(Debug)]
pub struct DbUserMarriage {
    /// Person who initiated the marriage
    pub proposer_id: i64,
    /// Person who accepted the marriage
    pub proposee_id: i64,
    /// Are they divorced
    pub divorced: bool,
    /// Was their marriage proposal rejected
    pub rejected: bool,
    /// What was the reason for marrying
    pub reason: String,
}
