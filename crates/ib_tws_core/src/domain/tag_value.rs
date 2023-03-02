use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TagValue {
    pub tag: String,
    pub value: String,
}

impl TagValue {
    pub fn new(tag: &str, value: &str) -> Self {
        TagValue {
            tag: tag.to_string(),
            value: value.to_string(),
        }
    }
}
