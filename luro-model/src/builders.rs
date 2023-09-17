pub mod components;
pub mod embed;
pub mod timestamp;

pub struct TimestampBuilder(twilight_model::util::Timestamp);

/// Based on Serenity's builders, but hopefully engineered for more correctness. This uses function builders to make creating embeds a little more erganomic, instead of the typical route of chaining methods together. Handy for short and simple commands. This is simply a wrapper around Twilight's ['Embed'], so you can consume these in exactly the same way without needing to run `.build()` or anything. You can also turn ['EmbedAuthorBuilder'] from `twilight-util` into these types as well!
///
/// Example:
/// ```rust
/// ctx.respond(|response| {
///     response.embed(|embed| {
///         embed.title("Hello World")
///             .description("I really like you!")
///             .color(0xDABEEF)
///         })
///     }).await;
/// ```

#[derive(Clone)]
pub struct EmbedBuilder(pub twilight_model::channel::message::Embed);

#[derive(Default)]
pub struct ComponentBuilder(Vec<twilight_model::channel::message::Component>);