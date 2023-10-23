use crate::{LuroGuild, LuroMember, LuroRole};

impl LuroGuild {
    /// Gets a position in [RolePosition] for what the user's highest role is.
    pub fn get_member_highest_role<'a>(&'a self, member: &'a Option<LuroMember>) -> Option<&'a LuroRole> {
        match member {
            Some(member) => member.sorted_roles().first().cloned(),
            None => None,
        }
    }
}
