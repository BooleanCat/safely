use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub id: String,
    pub id_type: String,
}

impl Device {
    pub fn new(id: &str, id_type: &str) -> Self {
        Self {
            id: String::from(id),
            id_type: String::from(id_type),
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
            "id": "24E552D7-6523-47F7-A647-D3465BF1F5CA",
            "idType": "class"
        });

        let got =
            serde_json::to_value(Device::new("24E552D7-6523-47F7-A647-D3465BF1F5CA", "class"))
                .unwrap();

        assert_eq!(want, got);
    }
}
