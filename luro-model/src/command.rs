mod prefix_command;
mod slash_command;

pub use self::prefix_command::PrefixCommand;
pub use self::slash_command::{SlashCommand, SlashError, SlashResult};

pub type InteractionSuccess = ();
pub type InteractionResponse = Result<InteractionSuccess, InteractionError>;

pub type ComponentCommand = ();
pub type ModalCommand = ();

pub type SlashContext = ();
pub type PrefixContext = ();
pub type ComponentContext = ();
pub type ModalContext = ();

pub enum Command {
    SlashCommand(SlashCommand),
    PrefixCommand(PrefixCommand),
}

impl Command {}

#[derive(Debug, thiserror::Error)]
pub enum InteractionError {
    // Bah
    #[error("This command cannot be used as it REQUIRES a database backend in order to function.")]
    RequiresDatabase,
    #[error("Only Component Interactions have this data")]
    NotComponent,
    #[error("PING interactions do not have author data")]
    NoAuthor,
    #[cfg(feature = "database-sqlx")]
    #[error("The database returned an error")]
    DatabaseError(#[from] sqlx::Error),
    #[error("No application data to create a command with!")]
    NoApplicationData,
    #[error("Interaction is not in a guild")]
    NotGuild,
    // #[error("Twilight failed to parse")]
    // ParseError(#[from] twilight_interactions::error::ParseError),
    #[error("Author not present within interaction")]
    AuthorNotPresent,
    #[error("Twilight failed to deserialize a response")]
    DeserializeBodyError(#[from] twilight_http::response::DeserializeBodyError),
    #[error("The API client had an error while communicating with the Discord API")]
    TwilightClient(#[from] twilight_http::Error),
    #[error("The command attempted to access data only available to components")]
    CommandFromComponent,
    // #[error("The bot is missing `{0:?}` permission to work.")]
    // BotMissingPermission(Permissions),
    // #[error("You are missing the `{0:?}` permission.")]
    // MissingPermission(Permissions),
    #[error("An attempt to modify a server owner in an unauthorised way")]
    ModifyServerOwner,
    #[error("The user is above you, and you are trying to do a privileged action.")]
    UserHeirarchy,
    #[error("The bot is not high enough in the role heirarchy to perform this request.")]
    BotHeirarchy,
    #[error("You are not marked as the bot owner.")]
    NotOwner,
    // #[error("A generic error was raised: {0}")]
    // Anyhow(#[from] std::error::Error),
    #[error("A formatting error was raised: {0}")]
    FmtError(#[from] std::fmt::Error),
}
