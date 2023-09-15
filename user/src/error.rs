use std::fmt;

pub enum Error {
    NoUsernameProvided,
}

impl fmt::Display for Error {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}
