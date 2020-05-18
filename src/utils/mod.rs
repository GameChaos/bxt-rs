//! Utility objects.

use std::{
    panic::{catch_unwind, UnwindSafe},
    process::abort,
};

pub mod marker;
pub use marker::MainThreadMarker;

pub mod function;
pub use function::Function;

pub mod variable;
pub use variable::Variable;

pub mod dl;

/// Runs the given function and aborts the process if it panics.
///
/// It's necessary to wrap the code of each hook in this function until Rust finally does this
/// automatically. https://github.com/rust-lang/rust/issues/52652
pub fn abort_on_panic<R, F: FnOnce() -> R + UnwindSafe>(f: F) -> R {
    match catch_unwind(f) {
        Ok(rv) => rv,
        Err(_) => abort(),
    }
}
