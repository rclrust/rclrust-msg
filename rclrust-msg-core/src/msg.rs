use std::os::raw::c_void;

use crate::traits::{FFIFromRust, FFIToRust};

pub trait MessageT: Default {
    type Raw: RawMessageT;
    type RawRef: RawMessageRefT;

    fn type_support() -> *const c_void;

    unsafe fn to_raw_ref(&self) -> Self::RawRef;
}

pub trait RawMessageT: FFIToRust + Default {}

pub trait RawMessageRefT: FFIFromRust {}
