use crate::field_type::{ConstantType, FieldType};

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub r#type: FieldType,
    pub default: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Constant {
    pub name: String,
    pub r#type: ConstantType,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct MsgSpec {
    pub package: String,
    pub name: String,
    pub fields: Vec<Field>,
    pub constants: Vec<Constant>,
}

#[derive(Debug, Clone)]
pub struct SrvSpec {
    pub package: String,
    pub name: String,
    pub request: MsgSpec,
    pub response: MsgSpec,
}

#[derive(Debug, Clone)]
pub struct ActionSpec {
    pub package: String,
    pub name: String,
    pub goal: MsgSpec,
    pub result: MsgSpec,
    pub feed: MsgSpec,
}
