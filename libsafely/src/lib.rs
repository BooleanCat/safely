pub mod config;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Parse(toml::de::Error),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error)
    }
}

impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Self {
        Error::Parse(error)
    }
}
