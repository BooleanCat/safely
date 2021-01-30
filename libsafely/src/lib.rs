pub mod config;

#[derive(Debug)]
pub enum Error {
    NotFound,
}

#[derive(Debug)]
pub struct State {}

pub fn create(_: config::Config) -> Result<(), Error> {
    Ok(())
}

pub fn state(_: &str) -> Result<State, Error> {
    Err(Error::NotFound)
}

pub fn delete(_: &str) -> Result<(), Error> {
    Err(Error::NotFound)
}
