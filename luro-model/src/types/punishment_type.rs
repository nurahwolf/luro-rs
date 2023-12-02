/// The type of punishment. Allows setting a reason
pub enum PunishmentType {
    /// The user was kicked. Allows setting a reason.
    Kicked(Option<String>),
    /// The user was unbanned. Allows setting a reason.
    Unbanned(Option<String>),
}
