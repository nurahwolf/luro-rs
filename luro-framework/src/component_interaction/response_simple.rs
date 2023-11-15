use luro_model::types::CommandResponse;

use crate::{standard_response::Response, ComponentInteraction};

impl ComponentInteraction {
    pub async fn response_simple(&self, response: Response<'_>) -> anyhow::Result<CommandResponse> {
        self.respond(|r| r.add_embed(response.embed())).await
    }
}
