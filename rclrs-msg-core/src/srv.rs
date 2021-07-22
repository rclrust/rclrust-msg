use std::os::raw::c_void;

use crate::msg::MessageT;

pub trait ServiceT {
    type Request: MessageT;
    type Response: MessageT;

    fn type_support() -> *const c_void;
}
