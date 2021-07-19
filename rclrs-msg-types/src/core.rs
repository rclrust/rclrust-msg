use crate::{ConstantType, MemberType};

/// A member of a structure
#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    /// The name of the member
    pub name: String,
    /// The type of the member
    pub r#type: MemberType,
    /// The default value of the member (optional)
    pub default: Option<String>,
}

/// A constant definition
#[derive(Debug, Clone)]
pub struct Constant {
    /// The name of the constant
    pub name: String,
    /// The type of the constant
    pub r#type: ConstantType,
    /// The value of the constant
    pub value: String,
}

/// A message definition
#[derive(Debug, Clone)]
pub struct Message {
    /// The package name
    pub package: String,
    /// The name of the message
    pub name: String,
    /// The list of the members
    pub members: Vec<Member>,
    /// The list of the constants
    pub constants: Vec<Constant>,
}

/// A service definition
#[derive(Debug, Clone)]
pub struct Service {
    /// The name of The package
    pub package: String,
    /// The name of the service
    pub name: String,
    /// The type of the request
    pub request: Message,
    /// The type of the response
    pub response: Message,
}

/// An action definition
#[derive(Debug, Clone)]
pub struct Action {
    /// The name of The package
    pub package: String,
    /// The name of The action
    pub name: String,
    /// The type of the goal
    pub goal: Message,
    /// The type of the result
    pub result: Message,
    /// The type of the feedback
    pub feedback: Message,
}
