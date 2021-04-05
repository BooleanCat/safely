mod hypervisor;
mod image;
mod kernel;

pub use hypervisor::Hypervisor;
pub use image::Image;
pub use kernel::Kernel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Vm {
    #[serde(skip_serializing_if = "Option::is_none")]
    hypervisor: Option<Hypervisor>,

    kernel: Kernel,

    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Image>,
}

impl Vm {
    pub fn new(kernel: Kernel) -> Self {
        Self {
            hypervisor: None,
            kernel,
            image: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Hypervisor, Image, Kernel, Vm};
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "kernel": {
                "path": "/path/to/vmlinuz"
            }
        });

        let got = serde_json::to_value(Vm::new(Kernel::new("/path/to/vmlinuz"))).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "hypervisor": {
                "path": "/path/to/vm"
            },
            "kernel": {
                "path": "/path/to/vmlinuz"
            },
            "image": {
                "path": "/path/to/vm/rootfs.img",
                "format": "raw"
            }
        });

        let got = serde_json::to_value(Vm {
            hypervisor: Some(Hypervisor::new("/path/to/vm")),
            image: Some(Image::new("/path/to/vm/rootfs.img", "raw")),
            ..Vm::new(Kernel::new("/path/to/vmlinuz"))
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
