mod anet;
mod capped_cpu;
mod capped_memory;

pub use anet::ANet;
pub use capped_cpu::CappedCPU;
pub use capped_memory::CappedMemory;
use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Solaris {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limitpriv: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_shm_memory: Option<String>,

    #[serde(rename = "cappedCPU", skip_serializing_if = "Option::is_none")]
    pub capped_cpu: Option<CappedCPU>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capped_memory: Option<CappedMemory>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub anet: Option<Vec<ANet>>,
}

impl Solaris {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::{ANet, CappedCPU, CappedMemory, Solaris};
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({});

        let got = serde_json::to_value(Solaris::new()).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "milestone": "svc:/milestone/container:default",
            "limitpriv": "default",
            "maxShmMemory": "512m",
            "cappedCPU": {},
            "cappedMemory": {},
            "anet": [
                {}
            ]
        });

        let got = serde_json::to_value(Solaris {
            milestone: Some(String::from("svc:/milestone/container:default")),
            limitpriv: Some(String::from("default")),
            max_shm_memory: Some(String::from("512m")),
            capped_cpu: Some(CappedCPU::new()),
            capped_memory: Some(CappedMemory::new()),
            anet: Some(vec![ANet::new()]),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
