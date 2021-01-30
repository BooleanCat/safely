use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Memory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel: Option<i64>,

    #[serde(rename = "kernelTCP", skip_serializing_if = "Option::is_none")]
    pub kernel_tcp: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub swappiness: Option<u64>,

    #[serde(rename = "disableOOMKiller", skip_serializing_if = "Option::is_none")]
    pub disable_oom_killer: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_hierarchy: Option<bool>,
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
            "limit": 536870912,
            "reservation": 536870912,
            "swap": 536870912,
            "kernel": -1,
            "kernelTCP": -1,
            "swappiness": 0,
            "disableOOMKiller": false,
            "useHierarchy": true
        });

        let got = serde_json::to_value(Memory {
            limit: Some(536870912),
            reservation: Some(536870912),
            swap: Some(536870912),
            kernel: Some(-1),
            kernel_tcp: Some(-1),
            swappiness: Some(0),
            disable_oom_killer: Some(false),
            use_hierarchy: Some(true),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
