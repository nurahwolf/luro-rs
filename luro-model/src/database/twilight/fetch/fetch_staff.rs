use crate::{database::Error, user::UserContext};

impl crate::database::twilight::Database {
    pub async fn fetch_staff(&self) -> Result<Vec<UserContext>, Error> {
        let mut staff = vec![];
        for staff_id in crate::BOT_OWNERS {
            staff.push(self.fetch_user(staff_id).await?);
        }

        Ok(staff)
    }
}
