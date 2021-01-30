use super::{Capabilities, ConsoleSize, Rlimit};
use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Process {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_size: Option<ConsoleSize>,

    pub cwd: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_line: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rlimits: Option<Vec<Rlimit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub apparmor_profile: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_new_privileges: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_score_adj: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    selinux_label: Option<String>,
}

impl Process {
    pub fn new(cwd: &str) -> Self {
        Self {
            terminal: None,
            console_size: None,
            cwd: String::from(cwd),
            env: None,
            args: None,
            command_line: None,
            rlimits: None,
            apparmor_profile: None,
            capabilities: None,
            no_new_privileges: None,
            oom_score_adj: None,
            selinux_label: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Capabilities, ConsoleSize, Process, Rlimit};
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "cwd": "/here"
        });

        let got = serde_json::to_value(Process::new("/here")).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "cwd": "/here",
            "terminal": true,
            "consoleSize": {
                "height": 256,
                "width": 256
            },
            "env": [
                "FOO=BAR"
            ],
            "args": [
                "man",
                "ls"
            ],
            "commandLine": "man ls",
            "rlimits": [
                {
                    "type": "RLIMIT_CPU",
                    "soft": 1024,
                    "hard": 2048
                }
            ],
            "apparmorProfile": "/some/profile",
            "capabilities": {},
            "noNewPrivileges": true,
            "oomScoreAdj": 10,
            "selinuxLabel": "foo"
        });

        let got = serde_json::to_value(Process {
            terminal: Some(true),
            console_size: Some(ConsoleSize::new(256, 256)),
            env: Some(vec![String::from("FOO=BAR")]),
            args: Some(vec![String::from("man"), String::from("ls")]),
            command_line: Some(String::from("man ls")),
            rlimits: Some(vec![Rlimit::new("RLIMIT_CPU", 1024, 2048)]),
            apparmor_profile: Some(String::from("/some/profile")),
            capabilities: Some(Capabilities::new()),
            no_new_privileges: Some(true),
            oom_score_adj: Some(10),
            selinux_label: Some(String::from("foo")),
            ..Process::new("/here")
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
