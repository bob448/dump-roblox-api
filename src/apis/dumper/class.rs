use serde::Deserialize;

/// Dumped luau class
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Class {
    pub Members: Vec<ClassMember>,
    pub MemoryCategory: String,
    pub Name: String,
    /// To get all of the Class' members,
    /// get all of the Superclasses and
    /// get all of their members until
    /// the Superclass is `<<<ROOT>>>`
    pub Superclass: String,
    #[serde(default)]
    /// Contains tags like `NotBrowsable` and `NotCreatable`
    /// 
    /// You can use `Class.is_not_browsable()` and `Class.is_not_createable()` to
    /// get a boolean value which represents the presence of those tags.
    pub Tags: Vec<String>,
}

impl Class {
    /// Returns if `self.Tags` contains `"NotBrowsable"`
    pub fn is_not_browsable(&self) -> bool {
        self.Tags.contains(&"NotBrowsable".to_string())
    }
    /// Returns if `self.Tags` contains `"NotCreatable"`
    pub fn is_not_createable(&self) -> bool {
        self.Tags.contains(&"NotCreatable".to_string())
    }
}

/// A function, property, or event.
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct ClassMember {
    /// Common ones are `Data`, `Parts`, `Behavior`, `State`, and others.
    /// Can be an empty string.
    #[serde(default)]
    pub Category: String,
    /// Either `"Function"`, `"Property"`, or `"Event"`
    pub MemberType: String,
    pub Name: String,
    /// Not parsed because it can either be a string or `SecurityInfo`
    pub Security: serde_json::Value,
    /// Can be empty.
    #[serde(default)]
    pub Serialization: Serialization,
    pub ThreadSafety: String,
    #[serde(default)]
    /// Contains the type of the value, like `bool` or `number`
    /// Can be empty.
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
    /// Can be `Class`, `Primitive`, `Enum`, `DataType`, and empty.
    pub Category: String,
    /// Common ones are `bool`, `string`, `int`, `int64`, `Instance` and more.
    /// Can be empty.
    pub Name: String,
}
