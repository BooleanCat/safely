use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Cpu {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<u16>,
}

impl Cpu {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::Cpu;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({});

        let got = serde_json::to_value(Cpu::new()).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "count": 2,
            "shares": 1024,
            "maximum": 5000
        });

        let got = serde_json::to_value(Cpu {
            count: Some(2),
            shares: Some(1024),
            maximum: Some(5000),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
