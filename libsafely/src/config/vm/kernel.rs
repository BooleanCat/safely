use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Kernel {
    pub path: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub initrd: Option<String>,
}

impl Kernel {
    pub fn new(path: &str) -> Self {
        Self {
            path: String::from(path),
            parameters: None,
            initrd: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Kernel;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "path": "/path/to/vmlinuz"
        });

        let got = serde_json::to_value(Kernel::new("/path/to/vmlinuz")).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "path": "/path/to/vmlinuz",
            "parameters": [
                "foo=bar"
            ],
            "initrd": "/path/to/initrd.img"
        });

        let got = serde_json::to_value(Kernel {
            parameters: Some(vec![String::from("foo=bar")]),
            initrd: Some(String::from("/path/to/initrd.img")),
            ..Kernel::new("/path/to/vmlinuz")
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
