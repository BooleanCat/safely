use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{env, fs, io};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Config {
    root: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            root: PathBuf::from("/").join("run").join("safely"),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, libsafely::Error> {
        match Self::home_config() {
            Err(_) => Ok(Default::default()),
            Ok(home) => match fs::read_to_string(&home) {
                Ok(content) => match toml::from_str(&content) {
                    Ok(config) => Ok(config),
                    Err(error) => Err(libsafely::Error::from(error)),
                },
                Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Default::default()),
                Err(error) => Err(libsafely::Error::from(error)),
            },
        }
    }

    pub fn from_file<P: AsRef<Path>>(p: P) -> Result<Self, libsafely::Error> {
        toml::from_str(&fs::read_to_string(p)?).map_err(From::from)
    }

    fn home_config() -> Result<PathBuf, env::VarError> {
        match env::var("HOME") {
            Err(error) => Err(error),
            Ok(path) => Ok(PathBuf::from(&path)
                .join(".config")
                .join("safely")
                .join("safely.toml")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use serial_test::serial;
    use std::path::PathBuf;
    use std::{env, fs};

    #[test]
    fn config_from_file_reads_values() {
        let work_dir = tempfile::tempdir().unwrap();

        let config: Config = toml::from_str(r#"root = '/foo/bar'"#).unwrap();
        let config_path = PathBuf::from(work_dir.path()).join("safely.toml");

        fs::write(&config_path, toml::to_vec(&config).unwrap()).unwrap();

        let got = Config::from_file(&config_path).unwrap();

        assert_eq!(config, got);
    }

    #[test]
    fn config_from_file_reads_values_with_defaults() {
        let workdir = tempfile::tempdir().unwrap();

        fs::write(workdir.path().join("safely.toml"), "").unwrap();

        let got = Config::from_file(workdir.path().join("safely.toml")).unwrap();
        let want: Config = toml::from_str(r#"root = '/run/safely'"#).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn config_load_loads_defaults() {
        let got = Config::load().unwrap();
        let want: Config = toml::from_str(r#"root = '/run/safely'"#).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    #[serial]
    fn config_load_loads_from_home() {
        // Store original value of $HOME
        let original = env::var("HOME");

        // Ensure we got $HOME or it wasn't set
        if let Err(error) = &original {
            assert_eq!(error, &env::VarError::NotPresent);
        }

        let work_dir = tempfile::tempdir().unwrap();
        let config_dir = PathBuf::from(work_dir.path().join(".config").join("safely"));

        fs::create_dir_all(&config_dir).unwrap();

        let config: Config = toml::from_str(r#"root = '/foo/bar'"#).unwrap();

        fs::write(
            &config_dir.join("safely.toml"),
            toml::to_vec(&config).unwrap(),
        )
        .unwrap();

        // Update $HOME to workdir
        env::set_var("HOME", work_dir.path());

        let got = Config::load();

        // Restore or unset $HOME
        match original {
            Ok(value) => env::set_var("HOME", value),
            Err(env::VarError::NotPresent) => env::remove_var("HOME"),
            Err(_) => unreachable!(), // Already checked for this above
        };

        assert_eq!(got.unwrap(), config);
    }
}
