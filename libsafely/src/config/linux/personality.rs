use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Personality {
    pub domain: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
}

impl Personality {
    pub fn new(domain: &str) -> Self {
        Self {
            domain: String::from(domain),
            flags: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Personality;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "domain": "LINUX"
        });

        let got = serde_json::to_value(Personality::new("LINUX")).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "domain": "LINUX",
            "flags": [
                "foo"
            ]
        });

        let got = serde_json::to_value(Personality {
            flags: Some(vec![String::from("foo")]),
            ..Personality::new("LINUX")
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
