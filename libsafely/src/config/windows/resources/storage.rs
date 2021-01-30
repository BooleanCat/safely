use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Storage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bps: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox_size: Option<u64>,
}

impl Storage {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::Storage;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({});

        let got = serde_json::to_value(Storage::new()).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "iops": 50,
            "bps": 100,
            "sandboxSize": 1000
        });

        let got = serde_json::to_value(Storage {
            iops: Some(50),
            bps: Some(100),
            sandbox_size: Some(1000),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
