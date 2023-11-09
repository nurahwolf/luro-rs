mod count_approvers;
mod count_total_approvers;
mod get_approvals;
mod update_marriage_approval;

pub struct DbUserMarriageApprovalsCount {
    pub approvers: Option<i64>,
    pub disapprovers: Option<i64>,
}
pub struct DbUserMarriageApprovals {
    pub user_id: i64,
    pub proposer_id: i64,
    pub proposee_id: i64,
    pub approve: bool,
    pub disapprove: bool,
}
