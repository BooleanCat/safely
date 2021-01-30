use serde::{Deserialize, Serialize};

#[serde(untagged)]
#[derive(Serialize, Deserialize, Debug)]
pub enum User {
    #[serde(rename_all = "camelCase")]
    Posix {
        uid: i32,
        gid: i32,

        #[serde(skip_serializing_if = "Option::is_none")]
        umask: Option<i32>,

        #[serde(skip_serializing_if = "Option::is_none")]
        additional_gids: Option<Vec<i32>>,
    },
    Windows {
        #[serde(skip_serializing_if = "Option::is_none")]
        username: Option<String>,
    },
}

impl User {
    pub fn posix(uid: i32, gid: i32) -> Self {
        User::Posix {
            uid,
            gid,
            umask: None,
            additional_gids: None,
        }
    }

    pub fn windows() -> Self {
        User::Windows { username: None }
    }
}

#[cfg(test)]
mod tests {
    use super::User;
    use serde_json;

    #[test]
    fn serialize_windows() {
        let want = serde_json::json!({});

        let got = serde_json::to_value(User::windows()).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_windows_optional_fields() {
        let want = serde_json::json!({
            "username": "pikachu"
        });

        let got = serde_json::to_value(User::Windows {
            username: Some(String::from("pikachu")),
        })
        .unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_posix() {
        let want = serde_json::json!({
            "uid": 1000,
            "gid": 2000
        });

        let got = serde_json::to_value(User::posix(1000, 2000)).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_posix_optional_fields() {
        let want = serde_json::json!({
            "uid": 1000,
            "gid": 2000,
            "umask": 0755,
            "additionalGids": [
                1,
                2,
                3
            ]
        });

        let got = serde_json::to_value(User::Posix {
            uid: 1000,
            gid: 2000,
            umask: Some(0755),
            additional_gids: Some(vec![1, 2, 3]),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
