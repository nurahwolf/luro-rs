use crate::{database::Error, user::MemberContext};

impl crate::database::Database {
    // Fetch a member from the database. Note that due to the need to query the database twice, this does not get roles automatically.
    pub async fn fetch_member_roles<'a>(&'a self, member: &'a mut MemberContext) -> Result<&'a mut MemberContext, Error> {
        #[cfg(feature = "database-sqlx")]
        match self.sqlx_driver.fetch_member_roles(member).await {
            true => return Ok(member),
            false => tracing::warn!("No roles returned from the database, so checking with Twilight."),
        }

        for role in self.twilight_client.roles(member.guild_id).await?.model().await? {
            match role.id.get() == member.guild_id.get() {
                true => {
                    member.twilight_member.roles.push(role.id);
                    member.everyone_role = Some(role)
                }
                false => {
                    member.twilight_member.roles.push(role.id);
                    member.roles.push(role)
                }
            }
        }

        member
            .roles
            .sort_by(|a, b| a.position.cmp(&b.position).then(a.id.get().cmp(&b.id.get())).reverse());

        Ok(member)
    }
}
