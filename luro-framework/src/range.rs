use crate::argument::ArgumentLimits;
use crate::builder::WrappedClient;
use crate::parse::{Parse, ParseError};
use crate::parse_impl::error;
use std::any::type_name;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::ops::{Deref, DerefMut};

mod sealed {
    use super::*;

    /// A trait used to specify the values [range](super::Range) can take.
    pub trait Number: Copy + Debug + Display {}

    macro_rules! number {
        ($($t:ty),* $(,)?) => {
            $(
                impl Number for $t {}
            )*
        };
    }

    number![i8, i16, i32, i64, isize, u8, u16, u32, u64, usize];
}

use async_trait::async_trait;
use sealed::Number;
use twilight_model::application::command::CommandOptionType;
use twilight_model::application::interaction::application_command::{
    CommandInteractionDataResolved, CommandOptionValue,
};

/// A range-like type used to constraint the input provided by the user. This is equivalent to
/// using a [RangeInclusive], but implements the [parse] trait.
///
/// [RangeInclusive]: std::ops::RangeInclusive
/// [parse]: Parse
#[derive(Copy, Clone)]
pub struct Range<T: Number, const START: i64, const END: i64>(T);

impl<T: Number, const START: i64, const END: i64> Deref for Range<T, START, END> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Number, const START: i64, const END: i64> DerefMut for Range<T, START, END> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[async_trait]
impl<T, E, const START: i64, const END: i64> Parse<T> for Range<E, START, END>
where
    T: Send + Sync,
    E: Parse<T> + Number,
{
    async fn parse(
        http_client: &WrappedClient,
        data: &T,
        value: Option<&CommandOptionValue>,
        resolved: Option<&mut CommandInteractionDataResolved>,
    ) -> Result<Self, ParseError> {
        let value = E::parse(http_client, data, value, resolved).await?;

        // SAFETY: Both the upper and lower values must be i64's and the Number trait is implemented
        // only for integer numbers, so casting the value to an i64 is safe to do.
        let v = unsafe { *(&value as *const E as *const i64) };

        if v < START || v > END {
            return Err(error(
                &format!("Range<{}, {}, {}>", type_name::<E>(), START, END),
                true,
                "Input out of range",
            ));
        }

        Ok(Self(value))
    }

    fn kind() -> CommandOptionType {
        E::kind()
    }

    fn limits() -> Option<ArgumentLimits> {
        use twilight_model::application::command::CommandOptionValue;
        Some(ArgumentLimits {
            min: Some(CommandOptionValue::Integer(START)),
            max: Some(CommandOptionValue::Integer(END)),
        })
    }
}

impl<T: Number, const START: i64, const END: i64> Debug for Range<T, START, END> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Range<{}, {}, {}>({})",
            type_name::<T>(),
            START,
            END,
            self.0
        )
    }
}
