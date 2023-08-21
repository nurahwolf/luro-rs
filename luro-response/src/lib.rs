use luro_builder::embed::EmbedBuilder;
use luro_model::COLOUR_DANGER;
use tracing::error;

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
pub fn not_implemented_embed() -> EmbedBuilder {
    error!("A call was made to a command which does not exist!");
    EmbedBuilder::default().title("Command Not Present").colour(COLOUR_DANGER).description("Whoa! You found a command that is still be worked on! Apologies, this is not ready yet. If you have seen this error a bunch of times... Maybe you might want to let my owner know?").clone()
}
