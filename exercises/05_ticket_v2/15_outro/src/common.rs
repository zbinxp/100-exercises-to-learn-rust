#[derive(Debug, thiserror::Error)]
pub enum TicketNewError {
    #[error("The title cannot be empty")]
    TitleCannotBeEmpty,
    #[error("The title cannot be longer than 50 bytes")]
    TitleTooLong,
    #[error("The description cannot be empty")]
    DescriptionCannotBeEmpty,
    #[error("The description cannot be longer than 500 bytes")]
    DescriptionTooLong,
    #[error("Invalid status")]
    InvalidStatus
}