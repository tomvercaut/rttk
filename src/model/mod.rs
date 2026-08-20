mod dose_type;

pub use dose_type::*;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("Unable convert string to DoseType.")]
    DoseTypeFromStr,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
