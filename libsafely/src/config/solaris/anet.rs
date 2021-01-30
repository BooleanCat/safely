use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ANet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_link: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub configure_allowed_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub defrouter: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_protection: Option<String>,
}

impl ANet {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::ANet;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({});

        let got = serde_json::to_value(ANet::new()).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "allowedAddress": "172.17.0.2/16",
            "configureAllowedAddress": "true",
            "defrouter": "172.17.0.1/16",
            "linkProtection": "mac-nospoof, ip-nospoof",
            "linkname": "net0",
            "lowerLink": "net2",
            "macAddress": "02:42:f8:52:c7:16"
        });

        let got = serde_json::to_value(ANet {
            linkname: Some(String::from("net0")),
            lower_link: Some(String::from("net2")),
            allowed_address: Some(String::from("172.17.0.2/16")),
            configure_allowed_address: Some(String::from("true")),
            defrouter: Some(String::from("172.17.0.1/16")),
            mac_address: Some(String::from("02:42:f8:52:c7:16")),
            link_protection: Some(String::from("mac-nospoof, ip-nospoof")),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
