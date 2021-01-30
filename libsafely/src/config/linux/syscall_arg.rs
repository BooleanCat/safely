use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct SyscallArg {
    pub index: u32,
    pub value: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_two: Option<u64>,

    pub op: String,
}

impl SyscallArg {
    pub fn new(index: u32, value: u64, op: &str) -> Self {
        Self {
            index,
            value,
            value_two: None,
            op: String::from(op),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SyscallArg;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "index": 1,
            "value": 45,
            "op": "SCMP_CMP_NE"
        });

        let got = serde_json::to_value(SyscallArg::new(1, 45, "SCMP_CMP_NE")).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "index": 1,
            "value": 45,
            "valueTwo": 56,
            "op": "SCMP_CMP_NE"
        });

        let got = serde_json::to_value(SyscallArg {
            value_two: Some(56),
            ..SyscallArg::new(1, 45, "SCMP_CMP_NE")
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
