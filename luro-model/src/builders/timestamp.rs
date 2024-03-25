use std::time::{SystemTime, UNIX_EPOCH};

use time::PrimitiveDateTime;

/// A timestamp builder, which takes several different types of time and converts it to a timestamp that Discord expects.
pub struct TimestampBuilder(pub PrimitiveDateTime);
pub enum TimestampBuilderError {}

impl Default for TimestampBuilder {
    fn default() -> Self {
        Self(PrimitiveDateTime::MIN)
    }
}

impl TimestampBuilder {
    // pub fn from_systemtime(&mut self, systemtime: SystemTime) -> anyhow::Result<&mut Self> {
    //     let seconds = systemtime.duration_since(UNIX_EPOCH)?.as_secs();
    //     self.0 = Timestamp::from_secs(seconds.try_into().context("Expected to convert u64 into i64")?)?;
    //     Ok(self)
    // }
}

// impl From<TimestampBuilder> for twilight_model::util::Timestamp {
//     /// Convert an embed author builder into an embed author.
//     ///
//     /// This is equivalent to calling [`EmbedAuthorBuilder::build`].
//     fn from(builder: TimestampBuilder) -> Self {
//         twilight_model::util::Timestamp(builder.0)
//     }
// }
