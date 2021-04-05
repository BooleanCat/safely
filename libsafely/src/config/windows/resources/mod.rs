mod cpu;
mod memory;
mod storage;

pub use cpu::Cpu;
pub use memory::Memory;
use serde::{Deserialize, Serialize};
pub use storage::Storage;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Resources {
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<Memory>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<Cpu>,

    #[serde(skip_serializing_if = "Option::is_none")]
    storage: Option<Storage>,
}

impl Resources {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::{Cpu, Memory, Resources, Storage};
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({});

        let got = serde_json::to_value(Resources::new()).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "memory": {},
            "cpu": {},
            "storage": {}
        });

        let got = serde_json::to_value(Resources {
            memory: Some(Memory::new()),
            cpu: Some(Cpu::new()),
            storage: Some(Storage::new()),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
