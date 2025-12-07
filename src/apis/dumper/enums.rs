use serde::Deserialize;

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
    pub Value: u32,
}
