use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidQueryString,
    Network(reqwest::Error),
    Referer,
    InvalidForm(serde_urlencoded::de::Error),
    CriticalServer(Box<dyn std::error::Error>),
    BackendConnection(reqwest::Error),
    BackendStatus(reqwest::Error),
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
    /// - API server sends an error
    /// - No connection to the server
    /// - Wrong json is recieved from the api server
    /// - TLS backend cannot be initialized 
    /// - Possibly others
    fn from(error: reqwest::Error) -> Self {
        if error.is_status() {
            Self::BackendStatus(error)
        } else if error.is_request() || error.is_connect() || error.is_redirect() {
            Self::BackendConnection(error)
        } else if error.is_builder() || error.is_decode() {
            Self::CriticalServer(Box::new(error))
        } else {
            Self::Network(error)
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        // TODO: Make link labels make sense out of context
        use Error::*;
        let headers =
            "\r\nContent-Type: text/html; charset=utf-8\r\nContent-Language: da\r\n\r\n\r\n"; // There could be a heading here
        let issue_tracker = "<a href=\"https://github.com/itzgoldenleonard/bullseyegolf-frontend-light/issues\">https://github.com/itzgoldenleonard/bullseyegolf-frontend-light/issues</a>";

        match self {
            CriticalServer(e) => {
                write!(f, "Status: 500{headers} Der skete en fejl på serveren, dette er en bug. Rapporter fejlen her: {issue_tracker} <br> Inkluder følgende informationer i fejlrapporten: Sidens URL, denne fejlbesked: <pre>{e}</pre>")
            }
            InvalidQueryString => {
                write!(f, "Status: 400{headers} Der er en fejl i URL'en. Tjek at du har stavet den rigtigt og at du har fået det rigtige link. <br> URL'en burde ende med: <pre>....org/u?u=<u>brugernavn</u></pre>")
            }
            Network(e) => {
                write!(f, "Status: 500{headers} Der skete en ukendt netværksfejl, dette er muligvis en bug. Fejlbesked: <pre>{e}</pre>")
            }
            BackendConnection(e) => {
                write!(f, "Status: 503\r\nRetry-After: 60{headers} Bullseyegolf light kunne ikke kommunikere med API serveren, prøv igen senere.<br>Fejlbesked:<pre>{e}</pre>")
            }
            BackendStatus(e) => {
                write!(f, "Status: 502{headers} Der skete en fejl på API serveren. <pre>{e}</pre>")
            }
            Referer => {
                write!(
                    f,
                    "Status: 400{headers} Bullseyegolf light kunne ikke afgøre hvilket hul din notering skal indsendes til. <br><br> Dette kan ske hvis du har indsendt din notering uden at vælge et hul først. Hvis det er tilfældet, så gå til hullet du ønsker at indsende noteringen til og vælg 'Indsend notering' <br><br>Dette kan også ske hvis din browsers referrer policy er for streng. Bullseyegolf light bruger Referer til at afgøre hvor din notering skal sendes hen og virker derfor ikke hvis din referer policy er indstillet til 'no-referrer'. Indstillingen vil kun være sat til 'no-referrer' hvis du selv har gjort det. Du kan prøve med en anden browser, eller nulstille indstillingen.<br><a href=\"https://www.technipages.com/firefox-enable-disable-referrer/\">Hvis du bruger firefox kan du læse hvordan du gør her</a><br><a href=\"https://developer.chrome.com/blog/referrer-policy-new-chrome-default/#test-the-change-and-figure-out-if-this-will-impact-your-site\">Hvis du bruger chromium (chrome, edge, Samsung internet, brave, vivaldi, opera osv.) kan du læse hvordan du gør her</a>."
                )
            }
            InvalidForm(e) => {
                write!(
                    f,
                    "Status: 400{headers} Dataen du har indsendt er ikke i det rigtige format, dette burde ikke ske. Luk siden og prøv igen.<br/> Fejlbesked:<pre>{e}</pre>"
                )
            }
        }
    }
}
