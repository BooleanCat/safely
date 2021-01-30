use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CPU {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_runtime: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    realtime_period: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cpus: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mems: Option<String>,
}

impl CPU {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::CPU;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({});

        let got = serde_json::to_value(CPU::new()).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "shares": 1024,
            "quota": 1000000,
            "period": 500000,
            "realtimeRuntime": 950000,
            "realtimePeriod": 1000000,
            "cpus": "2-3",
            "mems": "0-7"
        });

        let got = serde_json::to_value(CPU {
            shares: Some(1024),
            quota: Some(1000000),
            period: Some(500000),
            realtime_runtime: Some(950000),
            realtime_period: Some(1000000),
            cpus: Some(String::from("2-3")),
            mems: Some(String::from("0-7")),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
