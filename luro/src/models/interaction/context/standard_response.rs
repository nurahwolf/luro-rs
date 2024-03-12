use crate::{models::interaction::InteractionResult, responses::StandardResponse};

impl super::InteractionContext {
    pub async fn standard_response(&self, response: StandardResponse<'_>) -> InteractionResult<()> {
        self.respond(|r| r.add_embed(response.builder())).await
    }
}
