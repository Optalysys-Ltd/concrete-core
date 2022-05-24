use crate::backends::core::private::crypto::encoding::{Cleartext as ImplCleartext};
use crate::specification::entities::markers::CleartextKind;
use crate::specification::entities::{AbstractEntity, CleartextEntity};
use serde::{Serialize, Deserialize};

/// A structure representing a cleartext with 32 bits of precision.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cleartext32(pub(crate) ImplCleartext<u32>);

impl AbstractEntity for Cleartext32 {
    type Kind = CleartextKind;
}

impl CleartextEntity for Cleartext32 {}

/// A structure representing a cleartext with 64 bits of precision.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cleartext64(pub(crate) ImplCleartext<u64>);

impl AbstractEntity for Cleartext64 {
    type Kind = CleartextKind;
}

impl CleartextEntity for Cleartext64 {}


/// A structure representing a floating point cleartext with 64 bits of precision.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FloatCleartext64(pub(crate) ImplCleartext<f64>);

impl AbstractEntity for FloatCleartext64 {
    type Kind = CleartextKind;
}

impl CleartextEntity for FloatCleartext64 {}