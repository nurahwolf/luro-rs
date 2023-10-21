use crate::{standard_response::Response, ModalInteraction};

impl ModalInteraction {
    pub async fn response_simple(&self, response: Response<'_>) -> anyhow::Result<()> {
        self.respond(|r| r.add_embed(response.embed())).await
    }
}
