use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Priority {
    pub name: String,
    pub priority: u32,
}

impl Priority {
    pub fn new(name: &str, priority: u32) -> Self {
        Self {
            name: String::from(name),
            priority,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Priority;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "name": "eth0",
            "priority": 500
        });

        let got = serde_json::to_value(Priority::new("eth0", 500)).unwrap();

        assert_eq!(want, got);
    }
}
