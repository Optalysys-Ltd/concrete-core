#![deny(rustdoc::broken_intra_doc_links)]
//! A module containing backends correctness tests.
//!
//! Each submodule here is expected to be activated by a given feature flag (matching the
//! `backend_*` naming), and to contain the instantiation of a generic correctness test for every
//! implemented operator.
use concrete_core_fixture::{Repetitions, SampleSize};

/// The number of time a test is repeated for a single set of parameter.
pub const REPETITIONS: Repetitions = Repetitions(10);

/// The size of the sample used to perform statistical tests.
pub const SAMPLE_SIZE: SampleSize = SampleSize(100);

#[cfg(all(feature = "backend_cuda", not(feature = "_ci_do_not_compile")))]
pub mod cuda;
#[cfg(all(test, feature = "backend_default"))]
pub mod default;
#[cfg(all(test, feature = "backend_fft"))]
pub mod fft;
#[cfg(all(test, feature = "backend_fftw"))]
pub mod fftw;
