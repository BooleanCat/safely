use super::Syscall;
use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Seccomp {
    default_action: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    architectures: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    syscalls: Option<Vec<Syscall>>,
}

impl Seccomp {
    pub fn new(default_action: &str) -> Self {
        Self {
            default_action: String::from(default_action),
            architectures: None,
            flags: None,
            syscalls: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Seccomp, Syscall};
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "defaultAction": "SCMP_ACT_ALLOW"
        });

        let got = serde_json::to_value(Seccomp::new("SCMP_ACT_ALLOW")).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "defaultAction": "SCMP_ACT_ALLOW",
            "architectures": [
                "SCMP_ARCH_X86",
                "SCMP_ARCH_X32"
            ],
            "flags": [
                "SECCOMP_FILTER_FLAG_LOG"
            ],
            "syscalls": [
                {
                    "names": [
                        "getcwd"
                    ],
                    "action": "SCMP_ACT_ERRNO"
                }
            ]
        });

        let got = serde_json::to_value(Seccomp {
            architectures: Some(vec![
                String::from("SCMP_ARCH_X86"),
                String::from("SCMP_ARCH_X32"),
            ]),
            flags: Some(vec![String::from("SECCOMP_FILTER_FLAG_LOG")]),
            syscalls: Some(vec![Syscall::new(
                vec![String::from("getcwd")],
                "SCMP_ACT_ERRNO",
            )]),
            ..Seccomp::new("SCMP_ACT_ALLOW")
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
