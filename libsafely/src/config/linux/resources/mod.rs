mod block_io;
mod cpu;
mod device;
mod huge_page_limit;
mod memory;
mod network;
mod pids;
mod priority;
mod rdma;
mod throttle_device;
mod weight_device;

pub use block_io::BlockIo;
pub use cpu::Cpu;
pub use device::Device;
pub use huge_page_limit::HugePageLimit;
pub use memory::Memory;
pub use network::Network;
pub use pids::Pids;
pub use priority::Priority;
pub use rdma::Rdma;
pub use throttle_device::ThrottleDevice;
pub use weight_device::WeightDevice;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Resources {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<Memory>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<Cpu>,

    #[serde(rename = "blockIO", skip_serializing_if = "Option::is_none")]
    pub block_io: Option<BlockIo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub huge_page_limits: Option<Vec<HugePageLimit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pids: Option<Pids>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdma: Option<HashMap<String, Rdma>>,
}

impl Resources {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::{BlockIo, Cpu, Device, HugePageLimit, Memory, Network, Pids, Rdma, Resources};
    use serde_json;
    use std::collections::HashMap;

    #[test]
    fn serialize() {
        let want = serde_json::json!({});

        let got = serde_json::to_value(Resources::new()).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "devices": [
                {
                    "allow": true
                }
            ],
            "memory": {},
            "cpu": {},
            "blockIO": {},
            "hugePageLimits": [
                {
                    "pageSize": "2MB",
                    "limit": 209715200
                }
            ],
            "network": {},
            "pids": {
                "limit": 500
            },
            "rdma": {
                "mlx5_1": {}
            }
        });

        let mut rdma = HashMap::new();
        rdma.insert(String::from("mlx5_1"), Rdma::new());

        let got = serde_json::to_value(Resources {
            devices: Some(vec![Device::new(true)]),
            memory: Some(Memory::new()),
            cpu: Some(Cpu::new()),
            block_io: Some(BlockIo::new()),
            huge_page_limits: Some(vec![HugePageLimit::new("2MB", 209715200)]),
            network: Some(Network::new()),
            pids: Some(Pids::new(500)),
            rdma: Some(rdma),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
