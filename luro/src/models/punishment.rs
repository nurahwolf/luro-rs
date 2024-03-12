/// A punishment a user has received.
pub enum Punishment {
    /// The user was kicked. Allows setting a reason.
    Kicked(Option<String>),
    /// The user was unbanned. Allows setting a reason.
    Unbanned(Option<String>),
}
