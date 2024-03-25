impl super::InteractionContext {
    pub fn interaction_client(&self) -> twilight_http::client::InteractionClient {
        self.gateway.twilight_client.interaction(self.gateway.application.id)
    }
}
