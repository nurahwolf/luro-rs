use twilight_model::util::{Timestamp, datetime::TimestampParseError};

use crate::LuroMember;

impl LuroMember {
    pub fn communication_disabled_until(&self) -> Result<Option<Timestamp>, TimestampParseError> {
        Ok(match self.communication_disabled_until {
            Some(timestamp) => Some(Timestamp::from_secs(timestamp.unix_timestamp())?),
            None => None,
        })
    }
}