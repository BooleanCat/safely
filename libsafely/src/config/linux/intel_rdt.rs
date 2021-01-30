use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct IntelRDT {
    #[serde(rename = "closID", skip_serializing_if = "Option::is_none")]
    pub clos_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub l3_cache_schema: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_bw_schema: Option<String>,
}

impl IntelRDT {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::IntelRDT;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({});

        let got = serde_json::to_value(IntelRDT::new()).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "closID": "guaranteed_group",
            "l3CacheSchema": "L3:0=7f0;1=1f",
            "memBwSchema": "MB:0=20;1=70"
        });

        let got = serde_json::to_value(IntelRDT {
            clos_id: Some(String::from("guaranteed_group")),
            l3_cache_schema: Some(String::from("L3:0=7f0;1=1f")),
            mem_bw_schema: Some(String::from("MB:0=20;1=70")),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
