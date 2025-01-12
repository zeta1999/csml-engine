////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Event {
    pub content_type: String,
    pub content_value: String,
    pub content: serde_json::Value,
}

////////////////////////////////////////////////////////////////////////////////
// TRAIT FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Default for Event {
    fn default() -> Self {
        Self {
            content_type: String::default(),
            content_value: String::default(),
            content: serde_json::json!({}),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Event {
    pub fn new(content_type: &str, content_value: &str, content: serde_json::Value) -> Self {
        Self {
            content_type: content_type.to_owned(),
            content_value: content_value.to_owned(),
            content,
        }
    }
}
