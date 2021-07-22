#![warn(
    rust_2018_idioms,
    elided_lifetimes_in_paths,
    clippy::all,
    clippy::nursery
)]

pub mod action;
#[doc(hidden)]
pub mod ffi;
pub mod msg;
pub mod sequence;
pub mod srv;
pub mod string;
pub mod traits;

pub use sequence::{FFISeq, OwnedFFISeq, RefFFISeq};
pub use string::{FFIString, FFIWString, OwnedFFIString, OwnedFFIWString};
