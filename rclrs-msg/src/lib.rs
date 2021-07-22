#![warn(rust_2018_idioms, elided_lifetimes_in_paths)]
#![allow(clippy::all)]

pub use rclrs_msg_core::action::ActionT;
pub use rclrs_msg_core::msg::MessageT;
pub use rclrs_msg_core::srv::ServiceT;

include!(concat!(env!("OUT_DIR"), "/gen.rs"));
