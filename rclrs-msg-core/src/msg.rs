use std::os::raw::c_void;

use crate::traits::{FFIFromRust, FFIToRust};

pub trait MessageT {
    type Raw: RawMessageT;
    type RawRef: RawMessageRefT;

    fn type_support() -> *const c_void;

    unsafe fn to_raw_ref(&self) -> Self::RawRef;
}

pub trait RawMessageT: FFIToRust {}

pub trait RawMessageRefT: FFIFromRust {}
