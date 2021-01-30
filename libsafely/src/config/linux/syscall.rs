use super::SyscallArg;
use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Syscall {
    pub names: Vec<String>,
    pub action: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errno_ret: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<SyscallArg>>,
}

impl Syscall {
    pub fn new(names: Vec<String>, action: &str) -> Self {
        Self {
            names,
            action: String::from(action),
            errno_ret: None,
            args: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Syscall, SyscallArg};
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "names": [
                "getcwd",
                "chmod"
            ],
            "action": "SCMP_ACT_ERRNO"
        });

        let got = serde_json::to_value(Syscall::new(
            vec![String::from("getcwd"), String::from("chmod")],
            "SCMP_ACT_ERRNO",
        ))
        .unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "names": [
                "getcwd",
                "chmod"
            ],
            "action": "SCMP_ACT_ERRNO",
            "errnoRet": 4,
            "args": [
                {
                    "index": 1,
                    "value": 45,
                    "op": "SCMP_CMP_NE"
                }
            ]
        });

        let got = serde_json::to_value(Syscall {
            errno_ret: Some(4),
            args: Some(vec![SyscallArg::new(1, 45, "SCMP_CMP_NE")]),
            ..Syscall::new(
                vec![String::from("getcwd"), String::from("chmod")],
                "SCMP_ACT_ERRNO",
            )
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
