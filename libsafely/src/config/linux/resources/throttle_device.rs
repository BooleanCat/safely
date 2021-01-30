use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ThrottleDevice {
    pub major: i64,
    pub minor: i64,
    pub rate: u64,
}

impl ThrottleDevice {
    pub fn new(major: i64, minor: i64, rate: u64) -> Self {
        Self { major, minor, rate }
    }
}

#[cfg(test)]
mod tests {
    use super::ThrottleDevice;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "major": 10,
            "minor": 229,
            "rate": 50
        });

        let got = serde_json::to_value(ThrottleDevice::new(10, 229, 50)).unwrap();

        assert_eq!(want, got);
    }
}
