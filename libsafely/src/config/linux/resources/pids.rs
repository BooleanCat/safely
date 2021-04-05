use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pids {
    pub limit: i64,
}

impl Pids {
    pub fn new(limit: i64) -> Self {
        Self { limit }
    }
}

#[cfg(test)]
mod tests {
    use super::Pids;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "limit": 500
        });

        let got = serde_json::to_value(Pids::new(500)).unwrap();

        assert_eq!(want, got);
    }
}
