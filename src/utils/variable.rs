//! A main thread variable pointer.

use std::{cell::Cell, ffi::c_void, ptr::NonNull};

use crate::utils::MainThreadMarker;

/// Main thread variable pointer.
pub struct Variable<F> {
    ptr: Cell<Option<NonNull<F>>>,
}

// Safety: all methods are guarded with MainThreadMarker.
unsafe impl<F> Sync for Variable<F> {}

impl<F> Variable<F> {
    /// Creates an empty `Variable`.
    pub const fn empty() -> Self {
        Self {
            ptr: Cell::new(None),
        }
    }

    /// Resets the `Variable` to the empty state.
    pub fn reset(&self, _marker: MainThreadMarker) {
        self.ptr.set(None);
    }

    /// Retrieves the stored pointer.
    ///
    /// # Panics
    ///
    /// Panics if the `Variable` is empty.
    pub fn get(&self, _marker: MainThreadMarker) -> NonNull<F> {
        self.ptr.get().unwrap()
    }

    /// Sets the pointer.
    ///
    /// # Safety
    ///
    /// `ptr` must be a valid pointer of type `F` at least until the `Variable` is reset.
    pub unsafe fn set(&self, _marker: MainThreadMarker, ptr: Option<NonNull<c_void>>) {
        self.ptr.set(ptr.map(NonNull::cast));
    }

    /// Returns `true` if the `Variable` has a pointer stored.
    pub fn is_set(&self, _marker: MainThreadMarker) -> bool {
        self.ptr.get().is_some()
    }
}
