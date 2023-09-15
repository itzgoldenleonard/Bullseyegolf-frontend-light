use std::fmt;

pub enum Error {
    EnvVarReadError(&'static str, std::env::VarError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use Error::*;

        match self {
            EnvVarReadError(var_name, e) => {
                write!(f, "Status: 500\r\nContent-Type: text/html; charset=utf-8\r\n\r\n\r\nDer skete en fejl på serveren, dette er en bug. Rapporter fejlen med URL'en til denne side og denne information\n<pre>{var_name}: {e}</pre>") // TODO: Indicate danish language in the content-type header
            }
        }
    }
}
