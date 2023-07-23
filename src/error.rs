use anyhow::Error;

/// A wrapper around an anyhow error type
pub enum LuroError {
    /// An ephemeral error type
    EphemeralError(Error),
    /// An luro_response error type
    EphemeralDeferredError(Error),
    /// A regular error type
    Error(Error),
    /// A deferred error type
    DeferredError(Error)
}
