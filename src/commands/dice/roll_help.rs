use crate::{functions::guild_accent_colour::guild_accent_colour, Context, Error};

/// Show help for rolling dice
#[poise::command(slash_command, prefix_command, category = "Dice")]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    let description = "
            Roll some dice with a brief explanation of the output all on one line, such as `1d20 = [13] = 13`.
        ";

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
    let accent_colour = ctx.data().config.read().await.accent_colour;

    ctx.send(|reply| {
        reply.embed(|embed| {
            embed
                .title("Dice helper")
                .color(guild_accent_colour(accent_colour, ctx.guild()))
                .description(description)
                .field(shortmode_help[0], shortmode_help[1], false)
                .field(standard_help[0], standard_help[1], false)
                .field(percentile_help[0], percentile_help[1], false)
                .field(keep_help[0], keep_help[1], false)
                .field(drop_help[0], drop_help[1], false)
        })
    })
    .await?;
    Ok(())
}
