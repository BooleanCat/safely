use super::Priority;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Network {
    #[serde(rename = "classID", skip_serializing_if = "Option::is_none")]
    pub class_id: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priorities: Option<Vec<Priority>>,
}

impl Network {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::{Network, Priority};
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({});

        let got = serde_json::to_value(Network::new()).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "classID": 1048577,
            "priorities": [
                {
                    "name": "eth0",
                    "priority": 500
                }
            ]
        });

        let got = serde_json::to_value(Network {
            class_id: Some(1048577),
            priorities: Some(vec![Priority::new("eth0", 500)]),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
