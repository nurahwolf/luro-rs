use async_trait::async_trait;

/// A simple trait that implements a bunch of handy features in one place, such as getting a user's avatar. This can be included on other models to make getting date easier.
#[async_trait]
pub trait LuroFunctions {}
