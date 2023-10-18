mod count_marriages;
mod get_marriage;
mod get_marriages;
mod update_marriage;

#[derive(Debug)]
pub struct DbUserMarriage {
    pub proposer_id: i64,
    pub proposee_id: i64,
    pub active: bool,
    pub rejected: bool,
    pub reason: String,
}