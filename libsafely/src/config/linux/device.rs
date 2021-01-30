use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Device {
    #[serde(rename = "type")]
    pub device_type: String,

    pub path: String,
    pub major: i64,
    pub minor: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_mode: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<u32>,
}

impl Device {
    pub fn new(device_type: &str, path: &str, major: i64, minor: i64) -> Self {
        Self {
            device_type: String::from(device_type),
            path: String::from(path),
            major,
            minor,
            file_mode: None,
            uid: None,
            gid: None,
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
            "type": "c",
            "path": "/dev/fuse",
            "major": 10,
            "minor": 229
        });

        let got = serde_json::to_value(Device::new("c", "/dev/fuse", 10, 229)).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "type": "c",
            "path": "/dev/fuse",
            "major": 10,
            "minor": 229,
            "fileMode": 0755,
            "uid": 100,
            "gid": 200
        });

        let got = serde_json::to_value(Device {
            file_mode: Some(0755),
            uid: Some(100),
            gid: Some(200),
            ..Device::new("c", "/dev/fuse", 10, 229)
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
