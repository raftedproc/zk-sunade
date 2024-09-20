// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

mod constants;
mod errors;
mod groth_16;
mod verifier;

// entrypoint
pub use verifier::*;
