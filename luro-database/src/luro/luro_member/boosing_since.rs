use twilight_model::util::{datetime::TimestampParseError, Timestamp};

use crate::LuroMember;

impl LuroMember {
    pub fn boosting_since(&self) -> Result<Option<Timestamp>, TimestampParseError> {
        Ok(match self.boosting_since {
            Some(timestamp) => Some(Timestamp::from_secs(timestamp.unix_timestamp())?),
            None => None,
        })
    }
}
