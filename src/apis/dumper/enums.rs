use serde::Deserialize;

/// Contains EnumItems
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone)]
pub struct Enum {
    pub Items: Vec<EnumItem>,
    pub Name: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone)]
pub struct EnumItem {
    pub Name: String,
    /// The value of the `EnumItem`
    pub Value: u32,
}
