use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Hypervisor {
    pub path: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,
}

impl Hypervisor {
    pub fn new(path: &str) -> Self {
        Self {
            path: String::from(path),
            parameters: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Hypervisor;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "path": "/path/to/vm"
        });

        let got = serde_json::to_value(Hypervisor::new("/path/to/vm")).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "path": "/path/to/vm",
            "parameters": [
                "opts1=foo"
            ]
        });

        let got = serde_json::to_value(Hypervisor {
            parameters: Some(vec![String::from("opts1=foo")]),
            ..Hypervisor::new("/path/to/vm")
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
