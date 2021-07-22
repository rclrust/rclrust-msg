use std::os::raw::c_void;

use crate::msg::MessageT;
use crate::srv::ServiceT;

pub trait ActionT {
    type Goal: MessageT;
    type Result: MessageT;
    type Feedback: MessageT;
    type SendGoal: ServiceT;
    type GetResult: ServiceT;
    type FeedbackMessage: MessageT;

    fn type_support() -> *const c_void;
}
