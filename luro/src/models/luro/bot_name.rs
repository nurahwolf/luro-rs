impl super::Luro {
    pub async fn bot_name(&self) -> String {
        match &self.config.bot_name {
            Some(bot_name) => bot_name.clone(),
            None => self.current_user.name.clone(),
        }
    }
}
