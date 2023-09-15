use std::fmt;

#[derive(Debug)]
pub enum Error {
    EnvVarReadError(&'static str, std::env::VarError),
    InvalidQueryString,
    NetworkError, // TODO: Maybe this needs to be split up into multiple kinds of errors
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use Error::*;

        // TODO: Indicate danish language in the content-type header
        // TODO: Use appropriate error codes
        match self {
            EnvVarReadError(var_name, e) => {
                write!(f, "Status: 500\r\nContent-Type: text/html; charset=utf-8\r\n\r\n\r\nDer skete en fejl på serveren, dette er en bug. Rapporter fejlen med URL'en til denne side og denne information\n<pre>{var_name}: {e}</pre>")
            }
            InvalidQueryString => {
                write!(f, "Status: 400\r\nContent-Type: text/html; charset=utf-8\r\n\r\n\r\nDer er en fejl i URL'en. Tjek at du har stavet den rigtigt og at du har fået det rigtige link. <br/> URL'en burde ende med: <pre>....org/u?u=brugernavn</pre>")
            }
            NetworkError => {
                write!(f, "Status: 500\r\nContent-Type: text/html; charset=utf-8\r\n\r\n\r\nBullseyegolf light kunne ikke kommunikere med API serveren")
            }
        }
    }
}
