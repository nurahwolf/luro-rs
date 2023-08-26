use luro_builder::embed::EmbedBuilder;
use luro_model::COLOUR_DANGER;
use tracing::error;

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
pub fn not_implemented_embed() -> EmbedBuilder {
    error!("A call was made to a command that does not exist!");
    EmbedBuilder::default().title("Command Not Present").colour(COLOUR_DANGER).description("Looks like you managed to find a command that is actively being worked on, as it's not executable at present. If this error persists, might be best to let my owner know :)").clone()
}
