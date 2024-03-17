use thiserror::Error;

#[derive(Error, Debug)]
pub enum InputError {
    #[error("invalid input format")]
    InvalidFormat,
    #[error("invalid {0}")]
    InvalidType(String),
    #[error("unknown input error")]
    Unknown,
}
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("no first line found")]
    NoFirstLine,
    #[error("newcomer error")]
    Newcomer,
    #[error("citizenship error")]
    Citizenship,
    #[error("no citizens found")]
    NoCitizens,
    #[error("no customs found")]
    NoCustoms,
}
