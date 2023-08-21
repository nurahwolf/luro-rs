use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "help", desc = "Information for how to roll your dice")]
pub struct Help {
    /// Set your message to ephemeral, useful for if you don't want someone to see your rolls.
    ephemeral: Option<bool>
}

impl LuroCommand for Help {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let description = "Roll some dice with a brief explanation of the output all on one line, such as `1d20 = [13] = 13`.";

        let shortmode_help = [
            "Roll Example: /roll d8 + 2d4",
            "
```bash
d8 + 2d4 = [3] + [1, 4] = 8
```
    "
        ];

        let standard_help = [
            "Standard Rolls: /roll d",
            "
Standard notation allows you to roll any sided die any number of times
```bash
d     # roll a single 20 sided die
1d20  # equivalent
```
    "
        ];

        let percentile_help = [
            "Percentile Rolls: /roll 3d%",
            "
You can use `%` as a shorthand for 100 sides
```bash
3d%   # roll a percentile die 3 times and add them together
3d100 # equivalent
```
    "
        ];

        let keep_help = [
            "Keep Dice",
            "
The keep modifier allows you to roll multiple dice but only keep the highest or lowest result(s)
```bash
4d8kh2 # roll a d8 4 times and keep the highest 2 rolls
4d8k2  # equivalent to the above
4d8kl1 # roll a d10 4 times and keep the lowest roll
```
    "
        ];

        let drop_help = [
            "Drop Dice",
            "
The keep modifier allows you to roll multiple dice but drop the highest or lowest result(s). Opposite of Keep.
```bash
4d8dl2 # roll a d8 4 times and drop the lowest 2 rolls
4d8d2  # equivalent to the above
4d8dh1 # roll a d8 4 times and drop the highest roll
```
    "
        ];

        let accent_colour = ctx.accent_colour().await;
        ctx.respond(|r| {
            if self.ephemeral.unwrap_or_default() {
                r.ephemeral();
            }
            r.embed(|embed| {
                embed
                    .colour(accent_colour)
                    .title("Dice Helper")
                    .description(description)
                    .create_field(shortmode_help[0], shortmode_help[1], false)
                    .create_field(standard_help[0], standard_help[1], false)
                    .create_field(percentile_help[0], percentile_help[1], false)
                    .create_field(keep_help[0], keep_help[1], false)
                    .create_field(drop_help[0], drop_help[1], false)
            })
        })
        .await
    }
}
