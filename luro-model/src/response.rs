mod interaction_response;
#[cfg(feature = "builders")]
mod standard_response;

pub use interaction_response::InteractionResponse; // A response btype used for responding to interactions
#[cfg(feature = "builders")]
pub use standard_response::SimpleResponse; // A response type that wraps common responses, such as complaining we are not in a guild.

/// Safely find and truncate strings, lowers the number until we are no longer on a char boundary.
///
/// Used to ensure we don't send too much data to Discord.
pub fn safe_truncate(string: &mut String, mut new_len: usize) {
    while !string.is_char_boundary(new_len) {
        new_len -= 1;
    }

    string.truncate(new_len);
}

/// Safely find and split a string, lowers the number until we are no longer on a char boundary.
///
/// Used to ensure we don't send too much data to Discord.
pub fn safe_split(string: &str, mut split_at: usize) -> (&str, &str) {
    while !string.is_char_boundary(split_at) {
        split_at -= 1;
    }

    if split_at > string.len() {
        split_at = string.len()
    }

    string.split_at(split_at)
}
