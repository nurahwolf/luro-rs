impl super::Luro {
    pub fn interaction_client(&self) -> twilight_http::client::InteractionClient {
        self.twilight_client.interaction(self.application.id)
    }
}
