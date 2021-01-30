use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct IDMapping {
    #[serde(rename = "containerID")]
    pub container_id: u32,

    #[serde(rename = "hostID")]
    pub host_id: u32,

    pub size: u32,
}

impl IDMapping {
    pub fn new(container_id: u32, host_id: u32, size: u32) -> Self {
        Self {
            container_id,
            host_id,
            size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IDMapping;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "containerID": 0,
            "hostID": 2000,
            "size": 100
        });

        let got = serde_json::to_value(IDMapping::new(0, 2000, 100)).unwrap();

        assert_eq!(want, got);
    }
}
