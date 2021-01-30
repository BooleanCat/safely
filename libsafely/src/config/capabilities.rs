use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Capabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inheritable: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permitted: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ambient: Option<Vec<String>>,
}

impl Capabilities {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::Capabilities;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({});

        let got = serde_json::to_value(Capabilities::new()).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "effective": [
                "CAP_CHOWN"
            ],
            "bounding": [
                "CAP_DAC_OVERRIDE"
            ],
            "inheritable": [
                "CAP_DAC_READ_SEARCH",
                "CAP_FOWNER"
            ],
            "permitted": [
                "CAP_IPC_LOCK"
            ],
            "ambient": [
                "CAP_MKNOD"
            ]
        });

        let got = serde_json::to_value(Capabilities {
            effective: Some(vec![String::from("CAP_CHOWN")]),
            bounding: Some(vec![String::from("CAP_DAC_OVERRIDE")]),
            inheritable: Some(vec![
                String::from("CAP_DAC_READ_SEARCH"),
                String::from("CAP_FOWNER"),
            ]),
            permitted: Some(vec![String::from("CAP_IPC_LOCK")]),
            ambient: Some(vec![String::from("CAP_MKNOD")]),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
