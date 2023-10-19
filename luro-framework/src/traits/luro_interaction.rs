use std::future::Future;

use luro_model::{builders::EmbedBuilder, response::LuroResponse, user::LuroUser};
use twilight_interactions::command::ResolvedUser;
use twilight_model::{application::interaction::Interaction, channel::Message, id::{marker::{UserMarker, GuildMarker}, Id}, user::User};

use crate::Framework;

pub trait LuroInteraction {
    fn original_interaction(&self) -> &Interaction;
    fn accent_colour(&self, framework: &Framework) -> impl Future<Output = u32> + Send;
    fn acknowledge_interaction(
        &self,
        framework: &Framework,
        ephemeral: bool,
    ) -> impl Future<Output = anyhow::Result<LuroResponse>> + Send;
    fn default_embed(&self, framework: &Framework) -> impl Future<Output = EmbedBuilder> + Send;
    fn get_interaction_author(&self, framework: &Framework) -> impl Future<Output = anyhow::Result<LuroUser>> + Send;
    fn get_specified_user_or_author(
        &self,
        framework: &Framework,
        specified_user: Option<&ResolvedUser>,
    ) -> impl Future<Output = anyhow::Result<LuroUser>> + Send;
    fn respond_message<F>(&self, framework: &Framework, response: F) -> impl Future<Output = anyhow::Result<Option<Message>>> + Send
    where
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse;
    fn respond<F>(&self, framework: &Framework, response: F) -> impl Future<Output = anyhow::Result<()>> + Send
    where
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse;
    fn response_create(
        &self,
        framework: &Framework,
        response: &LuroResponse,
    ) -> impl Future<Output = anyhow::Result<Option<Message>>> + Send;
    fn response_update(&self, framework: &Framework, response: &LuroResponse) -> impl Future<Output = anyhow::Result<Message>> + Send;
    fn send_response(
        &self,
        framework: &Framework,
        response: LuroResponse,
    ) -> impl Future<Output = anyhow::Result<Option<Message>>> + Send;
    fn author_id(&self) -> Id<UserMarker>;
    fn author(&self) -> &User;
    fn guild_id(&self) -> Option<Id<GuildMarker>>;
    fn command_name(&self) -> &str;
}