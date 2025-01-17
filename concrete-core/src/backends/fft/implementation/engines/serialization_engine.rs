use crate::specification::engines::sealed::AbstractEngineSeal;
use crate::specification::engines::AbstractEngine;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum FftSerializationError {
    Serialization(bincode::Error),
    Deserialization(bincode::Error),
    UnsupportedVersion,
}

impl Display for FftSerializationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FftSerializationError::Serialization(bincode_error) => {
                write!(f, "Failed to serialize entity: {}", bincode_error)
            }
            FftSerializationError::Deserialization(bincode_error) => {
                write!(f, "Failed to deserialize entity: {}", bincode_error)
            }
            FftSerializationError::UnsupportedVersion => {
                write!(
                    f,
                    "The version used to serialize the entity is not supported."
                )
            }
        }
    }
}

impl Error for FftSerializationError {}

pub struct FftSerializationEngine;

impl AbstractEngineSeal for FftSerializationEngine {}
impl AbstractEngine for FftSerializationEngine {
    type EngineError = FftSerializationError;
    type Parameters = ();

    fn new(_parameters: Self::Parameters) -> Result<Self, Self::EngineError> {
        Ok(FftSerializationEngine)
    }
}
