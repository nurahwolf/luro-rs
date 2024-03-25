use crate::{database::Error, user::MemberContext};

impl crate::database::Database {
    // Fetch a member from the database. Note that due to the need to query the database twice, this does not get roles automatically.
    pub async fn fetch_member_roles<'a>(&'a self, member: &'a mut MemberContext) -> Result<&'a mut MemberContext, Error> {
        #[cfg(feature = "database-sqlx")]
        if !self.sqlx_driver.fetch_member_roles(member).await {
            tracing::warn!("No roles returned from the database, so checking with Twilight.");
        }

        self.twilight_driver.fetch_member_roles(member).await?;

        Ok(member)
    }
}
