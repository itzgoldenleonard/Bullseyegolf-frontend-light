use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidQueryString,
    Network, // TODO: Maybe this needs to be split up into multiple kinds of errors
    Referer,
    FormRead(std::io::Error),
    InvalidForm,
    CriticalServer(Box<dyn std::error::Error>),
}

impl From<serde_urlencoded::de::Error> for Error {
    fn from(_: serde_urlencoded::de::Error) -> Self {
        Self::InvalidQueryString
    }
}

impl From<std::env::VarError> for Error {
    fn from(error: std::env::VarError) -> Self {
        Self::CriticalServer(Box::new(error))
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(error: std::time::SystemTimeError) -> Self {
        Self::CriticalServer(Box::new(error))
    }
}

impl From<reqwest::Error> for Error {
    /// All possible types of network errors
    /// -
    fn from(error: reqwest::Error) -> Self {
        Self::Network
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use Error::*;
        let headers =
            "\r\nContent-Type: text/html; charset=utf-8\r\nContent-Language: da\r\n\r\n\r\n";
        let issue_tracker = "<a href=\"https://github.com/itzgoldenleonard/bullseyegolf-frontend-light/issues\">https://github.com/itzgoldenleonard/bullseyegolf-frontend-light/issues</a>";

        // TODO: Use appropriate error codes
        match self {
            CriticalServer(e) => {
                write!(f, "Status: 500{headers} Der skete en fejl på serveren, dette er en bug. Rapporter fejlen her: {issue_tracker} <br/> Inkluder følgende informationer i fejlrapporten: Sidens URL, denne fejlbesked: <pre>{e}</pre>")
            } // Finished
            InvalidQueryString => {
                write!(f, "Status: 400{headers} Der er en fejl i URL'en. Tjek at du har stavet den rigtigt og at du har fået det rigtige link. <br/> URL'en burde ende med: <pre>....org/u?u=<u>brugernavn</u></pre>")
            } // Finished
            Network => {
                write!(f, "Status: 500{headers} Bullseyegolf light kunne ikke kommunikere med API serveren")
            }
            Referer => {
                write!(
                    f,
                    "Status: 400{headers} Der er et problem med din browsers referrer policy"
                )
            }
            FormRead(e) => {
                write!(f, "Status: 500{headers} Der skete en fejl på serveren, dette er en bug. Rapporter fejlen med URL'en til denne side og denne information\n<pre>{e}</pre>")
            }
            InvalidForm => {
                write!(
                    f,
                    "Status: 400{headers} Dataen du har indsendt er ikke i det rigtige format"
                )
            }
        }
    }
}
