use std::fmt::Write;
use tracing::error;

use super::{drivers::LuroDatabaseDriver, LuroDatabase};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Flush ALL data held by the database. This will ensure all future hits go back to the raw driver
    ///
    /// NOTE: command_data and modal_data are NOT dropped, by design
    pub async fn flush(&self) -> anyhow::Result<String> {
        let mut errors = String::new();
        // TODO: application, staff and current user are not dropped
        match self.count.write() {
            Ok(mut write_lock) => *write_lock = 0,
            Err(why) => {
                error!(why = ?why, "count lock is poisoned");
                writeln!(errors, "{:#?}", why)?;
            }
        }

        match self.stories.write() {
            Ok(mut write_lock) => {
                write_lock.nsfw.clear();
                write_lock.sfw.clear();
            }
            Err(why) => {
                error!(why = ?why, "available_random_nsfw_hecks lock is poisoned");
                writeln!(errors, "{:#?}", why)?;
            }
        }

        match self.hecks.write() {
            Ok(mut write_lock) => {
                write_lock.nsfw.clear();
                write_lock.sfw.clear();
            }
            Err(why) => {
                error!(why = ?why, "available_random_sfw_hecks lock is poisoned");
                writeln!(errors, "{:#?}", why)?;
            }
        }

        match self.quotes.write() {
            Ok(mut write_lock) => {
                write_lock.clear();
            }
            Err(why) => {
                error!(why = ?why, "quotes lock is poisoned");
                writeln!(errors, "{:#?}", why)?;
            }
        }

        match self.guild_data.write() {
            Ok(mut write_lock) => {
                write_lock.clear();
            }
            Err(why) => {
                error!(why = ?why, "guild_data lock is poisoned");
                writeln!(errors, "{:#?}", why)?;
            }
        }

        match self.user_data.write() {
            Ok(mut write_lock) => {
                write_lock.clear();
            }
            Err(why) => {
                error!(why = ?why, "user_data lock is poisoned");
                writeln!(errors, "{:#?}", why)?;
            }
        }

        Ok(errors)
    }
}
