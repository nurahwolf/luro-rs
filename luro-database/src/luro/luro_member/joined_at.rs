use twilight_model::util::{datetime::TimestampParseError, Timestamp};

use crate::LuroMember;

impl LuroMember {
    pub fn joined_at(&self) -> Result<Timestamp, TimestampParseError> {
        Timestamp::from_secs(self.joined_at.unix_timestamp())
    }
}
