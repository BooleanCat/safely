use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub path: String,
    pub format: String,
}

impl Image {
    pub fn new(path: &str, format: &str) -> Self {
        Self {
            path: String::from(path),
            format: String::from(format),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Image;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "path": "/path/to/vm/rootfs.img",
            "format": "raw"
        });

        let got = serde_json::to_value(Image::new("/path/to/vm/rootfs.img", "raw")).unwrap();

        assert_eq!(want, got);
    }
}
