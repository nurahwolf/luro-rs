use luro_core::{Data, Error};

/// **Luro's error handler**
///
/// This function is called every time we have an error. There are many types of errors, so we only handle the ones we are particularly interested in. The rest get forwarded to the default error handler.
pub async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {error:?}"),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
            ctx.send(|message| {
                message
                    .ephemeral(true)
                    .content(format!("Error in command `{}`: {:?}", ctx.command().name, error))
            })
            .await
            .expect("Could not send error to channel!");
        }
        // We are not interested in this particular error, so handle it by the built-in function.
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {e}")
            }
        }
    }
}
