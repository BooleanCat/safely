use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CappedMemory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap: Option<String>,
}

impl CappedMemory {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::CappedMemory;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({});

        let got = serde_json::to_value(CappedMemory::new()).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "physical": "512m",
            "swap": "256m"
        });

        let got = serde_json::to_value(CappedMemory {
            physical: Some(String::from("512m")),
            swap: Some(String::from("256m")),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
