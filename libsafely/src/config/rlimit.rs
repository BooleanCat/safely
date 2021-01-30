use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Rlimit {
    #[serde(rename = "type")]
    pub rlimit_type: String,

    pub soft: u64,
    pub hard: u64,
}

impl Rlimit {
    pub fn new(rlimit_type: &str, soft: u64, hard: u64) -> Self {
        Self {
            rlimit_type: String::from(rlimit_type),
            soft,
            hard,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Rlimit;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "type": "RLIMIT_CPU",
            "soft": 1024,
            "hard": 2048
        });

        let got = serde_json::to_value(Rlimit::new("RLIMIT_CPU", 1024, 2048)).unwrap();

        assert_eq!(want, got);
    }
}
