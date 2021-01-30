use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub allow: bool,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
}

impl Device {
    pub fn new(allow: bool) -> Self {
        Self {
            allow,
            device_type: None,
            major: None,
            minor: None,
            access: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Device;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "allow": true
        });

        let got = serde_json::to_value(Device::new(true)).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "allow": true,
            "type": "c",
            "major": 10,
            "minor": 229,
            "access": "rvm"
        });

        let got = serde_json::to_value(Device {
            device_type: Some(String::from("c")),
            major: Some(10),
            minor: Some(229),
            access: Some(String::from("rvm")),
            ..Device::new(true)
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
