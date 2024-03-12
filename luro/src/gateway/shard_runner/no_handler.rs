pub fn no_handler(event: twilight_gateway::Event) {
    tracing::debug!(?event, "No handler for event");
}
