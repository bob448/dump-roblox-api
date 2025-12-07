use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Class {
    pub Members: Vec<ClassMember>,
    pub MemoryCategory: String,
    pub Name: String,
    pub Superclass: String,
    #[serde(default)]
    pub Tags: Vec<String>,
}

impl Class {
    pub fn is_not_browsable(&self) -> bool {
        self.Tags.contains(&"NotBrowsable".to_string())
    }
    pub fn is_not_createable(&self) -> bool {
        self.Tags.contains(&"NotCreatable".to_string())
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct ClassMember {
    #[serde(default)]
    pub Category: String,
    pub MemberType: String,
    pub Name: String,
    pub Security: serde_json::Value,
    #[serde(default)]
    pub Serialization: Serialization,
    pub ThreadSafety: String,
    #[serde(default)]
    pub ValueType: ValueType,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct SecurityInfo {
    pub Read: String,
    pub Write: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct Serialization {
    pub CanLoad: bool,
    pub CanSave: bool,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct ValueType {
    pub Category: String,
    pub Name: String,
}
