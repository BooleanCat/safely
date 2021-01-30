use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CappedCPU {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ncpus: Option<String>,
}

impl CappedCPU {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::CappedCPU;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({});

        let got = serde_json::to_value(CappedCPU::new()).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "ncpus": "8"
        });

        let got = serde_json::to_value(CappedCPU {
            ncpus: Some(String::from("8")),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
