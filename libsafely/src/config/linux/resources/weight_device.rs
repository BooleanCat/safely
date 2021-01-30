use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct WeightDevice {
    pub major: i64,
    pub minor: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf_weight: Option<u16>,
}

impl WeightDevice {
    pub fn new(major: i64, minor: i64) -> Self {
        Self {
            major,
            minor,
            weight: None,
            leaf_weight: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WeightDevice;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "major": 10,
            "minor": 229
        });

        let got = serde_json::to_value(WeightDevice::new(10, 229)).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "major": 10,
            "minor": 229,
            "weight": 50,
            "leafWeight": 100
        });

        let got = serde_json::to_value(WeightDevice {
            weight: Some(50),
            leaf_weight: Some(100),
            ..WeightDevice::new(10, 229)
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
