use luro_model::database::Database;

impl super::InteractionContext {
    /// Gets a reference to the internal database
    pub fn database(&self) -> &Database {
        &self.gateway.database
    }
}
