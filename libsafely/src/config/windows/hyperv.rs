use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct HyperV {
    #[serde(rename = "utilityVMPath", skip_serializing_if = "Option::is_none")]
    pub utility_vm_path: Option<String>,
}

impl HyperV {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::HyperV;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({});

        let got = serde_json::to_value(HyperV::new()).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "utilityVMPath": r#"C:\path\to\utilityvm"#
        });

        let got = serde_json::to_value(HyperV {
            utility_vm_path: Some(String::from(r#"C:\path\to\utilityvm"#)),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
