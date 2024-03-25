use crate::{
    database::twilight::{Database, Error},
    user::User,
};

impl Database {
    pub async fn fetch_staff(&self) -> Result<Vec<User>, Error> {
        let mut staff = vec![];
        for staff_id in crate::BOT_OWNERS {
            staff.push(self.fetch_user(staff_id).await?);
        }

        Ok(staff)
    }
}
