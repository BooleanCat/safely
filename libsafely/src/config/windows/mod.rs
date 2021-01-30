mod device;
mod hyperv;
mod network;
pub mod resources;

pub use device::Device;
pub use hyperv::HyperV;
pub use network::Network;
pub use resources::Resources;
use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Windows {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_folders: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Resources>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub servicing: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_flushes_during_boot: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyperv: Option<HyperV>,
}

impl Windows {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::{Device, HyperV, Network, Resources, Windows};
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({});

        let got = serde_json::to_value(Windows::new()).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "layerFolders": [
                r#"C:\Layers\layer2"#
            ],
            "devices": [
                {
                    "id": "5175d334-c371-4806-b3ba-71fd53c9258d",
                    "idType": "class"
                }
            ],
            "resources": {},
            "network": {},
            "servicing": false,
            "ignoreFlushesDuringBoot": true,
            "hyperv": {}
        });

        let got = serde_json::to_value(Windows {
            layer_folders: Some(vec![String::from(r#"C:\Layers\layer2"#)]),
            devices: Some(vec![Device::new(
                "5175d334-c371-4806-b3ba-71fd53c9258d",
                "class",
            )]),
            resources: Some(Resources::new()),
            network: Some(Network::new()),
            servicing: Some(false),
            ignore_flushes_during_boot: Some(true),
            hyperv: Some(HyperV::new()),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
