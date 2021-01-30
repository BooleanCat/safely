use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Memory {
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,
}

impl Memory {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::Memory;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({});

        let got = serde_json::to_value(Memory::new()).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "limit": 2097152
        });

        let got = serde_json::to_value(Memory {
            limit: Some(2097152),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
