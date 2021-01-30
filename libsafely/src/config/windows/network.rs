use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Network {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_list: Option<Vec<String>>,

    #[serde(
        rename = "allowUnqualifiedDNSQuery",
        skip_serializing_if = "Option::is_none"
    )]
    allow_unqualified_dns_query: Option<bool>,

    #[serde(rename = "DNSSearchList", skip_serializing_if = "Option::is_none")]
    dns_search_list: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    network_shared_container_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    network_namespace: Option<String>,
}

impl Network {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::Network;
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
            "endpointList": [
                "7a010682-17e0-4455-a838-02e5d9655fe6"
            ],
            "allowUnqualifiedDNSQuery": true,
            "DNSSearchList": [
                "a.com",
                "b.com"
            ],
            "networkSharedContainerName": "containerName",
            "networkNamespace": "168f3daf-efc6-4377-b20a-2c86764ba892"
        });

        let got = serde_json::to_value(Network {
            endpoint_list: Some(vec![String::from("7a010682-17e0-4455-a838-02e5d9655fe6")]),
            allow_unqualified_dns_query: Some(true),
            dns_search_list: Some(vec![String::from("a.com"), String::from("b.com")]),
            network_shared_container_name: Some(String::from("containerName")),
            network_namespace: Some(String::from("168f3daf-efc6-4377-b20a-2c86764ba892")),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
