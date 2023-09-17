use crate::{responses::Response, ModalInteraction};

impl<T> ModalInteraction<T> {
        pub async fn response_simple(&self, response: Response<'_>) -> anyhow::Result<()> {
        self.respond(|r| r.add_embed(response.embed())).await
    }
}
