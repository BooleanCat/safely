use super::Hook;
use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Hooks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prestart: Option<Vec<Hook>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_runtime: Option<Vec<Hook>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_container: Option<Vec<Hook>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_container: Option<Vec<Hook>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub poststart: Option<Vec<Hook>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub poststop: Option<Vec<Hook>>,
}

impl Hooks {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::{Hook, Hooks};
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({});

        let got = serde_json::to_value(Hooks::new()).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "prestart": [
                {
                    "path": "/bin/prestart"
                }
            ],
            "createRuntime": [
                {
                    "path": "/bin/runtime"
                }
            ],
            "createContainer": [
                {
                    "path": "/bin/container"
                }
            ],
            "startContainer": [
                {
                    "path": "/bin/start"
                }
            ],
            "poststart": [
                {
                    "path": "/bin/poststart"
                }
            ],
            "poststop": [
                {
                    "path": "/bin/poststop"
                }
            ]
        });

        let got = serde_json::to_value(Hooks {
            prestart: Some(vec![Hook::new("/bin/prestart")]),
            create_runtime: Some(vec![Hook::new("/bin/runtime")]),
            create_container: Some(vec![Hook::new("/bin/container")]),
            start_container: Some(vec![Hook::new("/bin/start")]),
            poststart: Some(vec![Hook::new("/bin/poststart")]),
            poststop: Some(vec![Hook::new("/bin/poststop")]),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
