/// Defines the preferred method of proxying messages that a user wishes to use. Falls back to other methods if the chosen method cannot be used.
pub enum ProxyMethod {
    BotEmbed,
    Webhook,
}
