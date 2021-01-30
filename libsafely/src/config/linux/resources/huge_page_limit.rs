use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct HugePageLimit {
    pub page_size: String,
    pub limit: u64,
}

impl HugePageLimit {
    pub fn new(page_size: &str, limit: u64) -> Self {
        Self {
            page_size: String::from(page_size),
            limit,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HugePageLimit;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "pageSize": "2MB",
            "limit": 209715200
        });

        let got = serde_json::to_value(HugePageLimit::new("2MB", 209715200)).unwrap();

        assert_eq!(want, got);
    }
}
