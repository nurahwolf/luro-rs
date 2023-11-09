use luro_model::sync::MemberSync;

impl crate::Database {
    pub async fn member_update(&self, member: impl Into<MemberSync>) -> anyhow::Result<u64> {
        self.driver.update_member(member).await
    }
}