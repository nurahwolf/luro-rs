use anyhow::Error;

use twilight_http::client::InteractionClient;

use twilight_model::http::interaction::InteractionResponse;

use twilight_model::{application::interaction::Interaction, http::interaction::InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;

/// A simple function to respond with `ChannelMessageWithSource`
pub async fn respond_to_interaction(
    interaction_client: &InteractionClient<'_>,
    interaction: &Interaction,
    content: String,
) -> Result<(), Error> {
    let data = InteractionResponseDataBuilder::new().content(content).build();

    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(data),
    };

    interaction_client
        .create_response(interaction.id, &interaction.token, &response)
        .await?;

    Ok(())
}

/// Work out how many padding characters is needed for a nicely formatted table.
/// This takes a vector containing the word / number lengths in base10, and provices you with the lenth
/// This is broken up by the length of the prefix, suffix and together.
fn padding_calculator(input: Vec<(usize, usize)>) -> (usize, usize, usize) {
    let mut prefix_length = 0;
    let mut suffix_length = 0;

    for (prefix, suffix) in input {
        if prefix > prefix_length {
            prefix_length = prefix
        }

        if suffix > suffix_length {
            suffix_length = suffix
        }
    }

    (prefix_length, suffix_length, prefix_length + suffix_length)
}
