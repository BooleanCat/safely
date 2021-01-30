use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConsoleSize {
    pub height: u32,
    pub width: u32,
}

impl ConsoleSize {
    pub fn new(height: u32, width: u32) -> Self {
        Self { height, width }
    }
}

#[cfg(test)]
mod tests {
    use super::ConsoleSize;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "height": 1024,
            "width": 620
        });

        let got = serde_json::to_value(ConsoleSize::new(1024, 620)).unwrap();

        assert_eq!(want, got);
    }
}
