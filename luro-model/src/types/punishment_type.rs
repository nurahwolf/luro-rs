/// The type of punishment. Allows setting a reason
pub enum PunishmentType<'a> {
    /// The user was kicked. Allows setting a reason.
    Kicked(Option<String>),
    /// The user was banned. First paramater is an optional reason, followed by the length of ban
    Banned(Option<&'a str>, i64),
    /// The user was unbanned. Allows setting a reason.
    Unbanned(Option<String>),
}
