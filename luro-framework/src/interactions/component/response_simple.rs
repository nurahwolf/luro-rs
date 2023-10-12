use crate::{responses::Response, ComponentInteraction};

impl ComponentInteraction {
    pub async fn response_simple(&self, response: Response<'_>) -> anyhow::Result<()> {
        self.respond(|r| r.add_embed(response.embed())).await
    }
}
