use crate::{standard_response::Response, CommandInteraction};

impl CommandInteraction {
    pub async fn response_simple(&self, response: Response<'_>) -> anyhow::Result<()> {
        self.respond(|r| r.add_embed(response.embed())).await
    }
}
