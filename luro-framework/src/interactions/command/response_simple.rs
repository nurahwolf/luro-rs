use crate::{responses::Response, CommandInteraction};

impl<T> CommandInteraction<T> {
    pub async fn response_simple(&self, response: Response<'_>) -> anyhow::Result<()> {
        self.respond(|r| r.add_embed(response.embed())).await
    }
}
