use twilight_model::application::interaction::Interaction;

impl crate::Database {
    pub async fn interaction_update(&self, interaction: &Interaction) -> anyhow::Result<u64> {
        self.driver.update_interaction(interaction).await
    }
}
