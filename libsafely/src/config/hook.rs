use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Hook {
    pub path: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}

impl Hook {
    pub fn new(path: &str) -> Self {
        Self {
            path: String::from(path),
            args: None,
            env: None,
            timeout: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Hook;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "path": "/usr/bin/fix-mounts"
        });

        let got = serde_json::to_value(Hook::new("/usr/bin/fix-mounts")).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "path": "/usr/bin/fix-mounts",
            "args": [
                "fix-mounts"
            ],
            "env": [
                "key1=value1"
            ],
            "timeout": 60
        });

        let got = serde_json::to_value(Hook {
            args: Some(vec![String::from("fix-mounts")]),
            env: Some(vec![String::from("key1=value1")]),
            timeout: Some(60),
            ..Hook::new("/usr/bin/fix-mounts")
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
