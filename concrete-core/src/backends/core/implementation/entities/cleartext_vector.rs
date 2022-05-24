use crate::backends::core::private::crypto::encoding::CleartextList as ImplCleartextList;
use crate::specification::entities::markers::CleartextVectorKind;
use crate::specification::entities::{AbstractEntity, CleartextVectorEntity};
use concrete_commons::parameters::CleartextCount;
use serde::{Serialize, Deserialize};

/// A structure representing a vector of cleartexts with 32 bits of precision.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CleartextVector32(pub(crate) ImplCleartextList<Vec<u32>>);

impl AbstractEntity for CleartextVector32 {
    type Kind = CleartextVectorKind;
}

impl CleartextVectorEntity for CleartextVector32 {
    fn cleartext_count(&self) -> CleartextCount {
        self.0.count()
    }
}

/// A structure representing a vector of cleartexts with 64 bits of precision.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CleartextVector64(pub(crate) ImplCleartextList<Vec<u64>>);

impl AbstractEntity for CleartextVector64 {
    type Kind = CleartextVectorKind;
}

impl CleartextVectorEntity for CleartextVector64 {
    fn cleartext_count(&self) -> CleartextCount {
        self.0.count()
    }
}

/// A structure representing a vector floating point cleartext with 64 bits of precision.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FloatCleartextVector64(pub(crate) ImplCleartextList<Vec<f64>>);

impl AbstractEntity for FloatCleartextVector64 {
    type Kind = CleartextVectorKind;
}

impl CleartextVectorEntity for FloatCleartextVector64 {
    fn cleartext_count(&self) -> CleartextCount {
        self.0.count()
    }
}