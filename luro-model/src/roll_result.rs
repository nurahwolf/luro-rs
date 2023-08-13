use core::fmt;

use crate::roll_value::RollValue;

pub struct RollResult {
    pub string_result: String,
    pub dice_total: RollValue
}

impl fmt::Display for RollResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string_result)
    }
}
