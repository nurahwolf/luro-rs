#[derive(Debug)]
pub enum LuroError {
    NoInteractionData,
    NoApplicationCommand,
    NoMessageInteractionData
}

impl std::error::Error for LuroError {}

impl fmt::Display for LuroError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LuroError::NoMessageInteractionData => write!(f, "No Message Interaction Data"),
            LuroError::NoInteractionData => write!(f, "No data was found in the interaction"),
            LuroError::NoApplicationCommand => write!(
                f,
                "No ApplicationCommand was found in the interaction's data"
            ),
        }
    }
}