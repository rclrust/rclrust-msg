use std::{
    ffi::{CStr, CString},
    mem::ManuallyDrop,
    os::raw::c_char,
};

use crate::ffi::uint_least16_t;
use crate::traits::{FFIFromRust, FFIToRust, ZeroInit};

/// An array of 8-bit characters terminated by a null character.
#[repr(C)]
#[derive(Debug)]
pub struct FFIString {
    data: *mut c_char,
    size: usize,
    capacity: usize,
}

impl FFIString {
    /// Returns the length of the string (excluding the null byte)
    pub const fn len(&self) -> usize {
        self.size
    }

    /// Returns `true` if the string has a length of 0.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl ZeroInit for FFIString {
    fn zero_init() -> Self {
        Self {
            data: std::ptr::null_mut(),
            size: 0,
            capacity: 0,
        }
    }
}

impl FFIToRust for FFIString {
    type Target = String;

    fn to_rust(&self) -> Self::Target {
        if self.is_empty() {
            "".to_string()
        } else {
            unsafe { CStr::from_ptr(self.data) }
                .to_str()
                .expect("CStr::to_str failed")
                .to_string()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct OwnedFFIString {
    data: *mut c_char,
    size: usize,
    capacity: usize,
}

impl OwnedFFIString {
    /// Returns the length of the string (excluding the null byte)
    pub const fn len(&self) -> usize {
        self.size
    }

    /// Returns `true` if the string has a length of 0.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl ZeroInit for OwnedFFIString {
    fn zero_init() -> Self {
        Self {
            data: std::ptr::null_mut(),
            size: 0,
            capacity: 0,
        }
    }
}

impl FFIFromRust for OwnedFFIString {
    type From = String;

    unsafe fn from_rust(string: &Self::From) -> Self {
        let cstring = CString::new(string.clone()).expect("CString::new failed");
        let len = cstring.as_bytes().len();
        Self {
            data: cstring.into_raw(),
            size: len,
            capacity: len + 1,
        }
    }
}

impl Drop for OwnedFFIString {
    fn drop(&mut self) {
        unsafe {
            CString::from_raw(self.data);
        }
    }
}

/// An array of 16-bit characters terminated by a null character.  <br>
/// *Is it better to be compatible with some crates supporting wide string?*
#[repr(C)]
#[derive(Debug)]
pub struct FFIWString {
    data: *mut uint_least16_t,
    size: usize,
    capacity: usize,
}

impl FFIWString {
    /// Returns the length of the string (excluding the null byte)
    pub const fn len(&self) -> usize {
        self.size
    }

    /// Returns `true` if the sequence has a length of 0.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl ZeroInit for FFIWString {
    fn zero_init() -> Self {
        Self {
            data: std::ptr::null_mut(),
            size: 0,
            capacity: 0,
        }
    }
}

impl FFIToRust for FFIWString {
    type Target = Vec<u16>;

    fn to_rust(&self) -> Self::Target {
        unsafe { std::slice::from_raw_parts(self.data, self.len()) }
            .iter()
            .map(|&v| v as u16)
            .collect::<Vec<_>>()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct OwnedFFIWString {
    data: *mut uint_least16_t,
    size: usize,
    capacity: usize,
}

impl OwnedFFIWString {
    /// Returns the length of the string (excluding the null byte)
    pub const fn len(&self) -> usize {
        self.size
    }

    /// Returns `true` if the sequence has a length of 0.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl ZeroInit for OwnedFFIWString {
    fn zero_init() -> Self {
        Self {
            data: std::ptr::null_mut(),
            size: 0,
            capacity: 0,
        }
    }
}

impl FFIFromRust for OwnedFFIWString {
    type From = Vec<u16>;

    unsafe fn from_rust(string: &Self::From) -> Self {
        let mut string = string.clone();
        string.push(0);
        string.shrink_to_fit();
        assert_eq!(string.len(), string.capacity());
        let mut string = ManuallyDrop::new(string);
        Self {
            data: string.as_mut_ptr(),
            size: string.len() - 1,
            capacity: string.len(),
        }
    }
}

impl Drop for OwnedFFIWString {
    fn drop(&mut self) {
        unsafe { Vec::from_raw_parts(self.data, self.capacity, self.capacity) };
    }
}
