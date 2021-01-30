use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Namespace {
    #[serde(rename = "type")]
    pub namespace_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl Namespace {
    pub fn new(namespace_type: &str) -> Self {
        Self {
            namespace_type: String::from(namespace_type),
            path: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Namespace;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "type": "pid"
        });

        let got = serde_json::to_value(Namespace::new("pid")).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "type": "pid",
            "path": "/namespaces/pid"
        });

        let got = serde_json::to_value(Namespace {
            path: Some(String::from("/namespaces/pid")),
            ..Namespace::new("pid")
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
