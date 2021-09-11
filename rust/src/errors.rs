use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("result of parsing test cases must be a valid top level suite")]
    SuiteMissingError,
    #[error("error serializing test cases to JSON")]
    JSONSerializationError(#[from] serde_json::Error),
}
