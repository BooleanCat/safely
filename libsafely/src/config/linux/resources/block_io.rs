use super::{ThrottleDevice, WeightDevice};
use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BlockIo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf_weight: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_device: Option<Vec<WeightDevice>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_read_bps_device: Option<Vec<ThrottleDevice>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_write_bps_device: Option<Vec<ThrottleDevice>>,

    #[serde(
        rename = "throttleReadIOPSDevice",
        skip_serializing_if = "Option::is_none"
    )]
    pub throttle_read_iops_device: Option<Vec<ThrottleDevice>>,

    #[serde(
        rename = "throttleWriteIOPSDevice",
        skip_serializing_if = "Option::is_none"
    )]
    pub throttle_write_iops_device: Option<Vec<ThrottleDevice>>,
}

impl BlockIo {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::{BlockIo, ThrottleDevice, WeightDevice};
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({});

        let got = serde_json::to_value(BlockIo::new()).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "weight": 50,
            "leafWeight": 100,
            "weightDevice": [
                {
                    "major": 10,
                    "minor": 229
                }
            ],
            "throttleReadBpsDevice": [
                {
                    "major": 10,
                    "minor": 229,
                    "rate": 10
                }
            ],
            "throttleWriteBpsDevice": [
                {
                    "major": 10,
                    "minor": 229,
                    "rate": 12
                }
            ],
            "throttleReadIOPSDevice": [
                {
                    "major": 10,
                    "minor": 229,
                    "rate": 11
                }
            ],
            "throttleWriteIOPSDevice": [
                {
                    "major": 10,
                    "minor": 229,
                    "rate": 17
                }
            ]
        });

        let got = serde_json::to_value(BlockIo {
            weight: Some(50),
            leaf_weight: Some(100),
            weight_device: Some(vec![WeightDevice::new(10, 229)]),
            throttle_read_bps_device: Some(vec![ThrottleDevice::new(10, 229, 10)]),
            throttle_write_bps_device: Some(vec![ThrottleDevice::new(10, 229, 12)]),
            throttle_read_iops_device: Some(vec![ThrottleDevice::new(10, 229, 11)]),
            throttle_write_iops_device: Some(vec![ThrottleDevice::new(10, 229, 17)]),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
