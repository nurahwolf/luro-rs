use twilight_util::builder::embed::EmbedBuilder;

use crate::{interactions::InteractionResponse, COLOUR_DANGER};

/// Luro is missing permissions in order to use this command.
pub fn bot_missing_permission(permission_missing: String) -> InteractionResponse {
    let embed = EmbedBuilder::new()
        .color(COLOUR_DANGER)
        .title("I am missing permissions")
        .description(format!("***SOME*** motherfucker failed to set me up correctly.\nI should have ***Administrator*** privileges in the server to work my best, but it seems I'm missing that. Fix it >:c\nIf you explicitly want to limit my permissions, I'm missing the {permission_missing} permisison for this command to work."))
        .build();

    InteractionResponse::Embed {
        embeds: vec![embed],
        components: None,
        ephemeral: true,
    }
}
