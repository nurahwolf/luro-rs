impl crate::ModalInteraction {
    pub async fn simple_respponse(
        &self,
        response: luro_model::response::SimpleResponse<'_>,
    ) -> anyhow::Result<luro_model::types::CommandResponse> {
        self.respond(|r| r.add_embed(response.embed())).await
    }
}
