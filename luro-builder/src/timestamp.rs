use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Context;
use twilight_model::util::Timestamp;

pub struct TimestampBuilder(Timestamp,);

impl TimestampBuilder {
    pub fn from_systemtime(&mut self, systemtime: SystemTime,) -> anyhow::Result<&mut Self,> {
        let seconds = systemtime.duration_since(UNIX_EPOCH,)?.as_secs();
        self.0 = Timestamp::from_secs(seconds.try_into().context("Expected to convert u64 into i64",)?,)?;
        Ok(self,)
    }
}

impl From<TimestampBuilder,> for Timestamp {
    /// Convert an embed author builder into an embed author.
    ///
    /// This is equivalent to calling [`EmbedAuthorBuilder::build`].
    fn from(builder: TimestampBuilder,) -> Self {
        builder.0
    }
}
