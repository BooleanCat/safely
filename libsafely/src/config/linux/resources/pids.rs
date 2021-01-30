use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PIDs {
    pub limit: i64,
}

impl PIDs {
    pub fn new(limit: i64) -> Self {
        Self { limit }
    }
}

#[cfg(test)]
mod tests {
    use super::PIDs;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "limit": 500
        });

        let got = serde_json::to_value(PIDs::new(500)).unwrap();

        assert_eq!(want, got);
    }
}
