use crate::error::Error;
use crate::get::Score;
use serde::Deserialize;
use serde_urlencoded as qs;
use std::env;
use std::io::stdin;

/// Main entrypoint for score submission
pub fn post() -> Result<String, Error> {
    let params = Params::new()?;

    let mut score_buffer = String::new();
    let _ = stdin().read_line(&mut score_buffer).map_err(Error::FormRead)?;
    let score: CustomScore = qs::from_str(&score_buffer).map_err(|_| Error::InvalidForm)?; // TODO: Try with from_reader
    submit_score(&params, score.into())?;
    Ok(format!(
        "Status: 303\r\nLocation: ?u={}&t={}&h={}\r\n\r\n\r\n",
        params.query_args.user, params.query_args.tournament, params.query_args.hole
    ))
}

struct Params {
    server: String,
    query_args: QueryParams,
}

#[derive(Debug, Deserialize)]
struct QueryParams {
    #[serde(rename = "u")]
    user: String,
    #[serde(rename = "t")]
    tournament: String,
    #[serde(rename = "h")]
    hole: u8,
}

impl Params {
    pub fn new() -> Result<Self, Error> {
        let server = env::var("SERVER_URL").map_err(|e| Error::EnvVarRead("SERVER_URL", e))?;
        let query_string = env::var("HTTP_REFERER").map_err(|_| Error::Referer)?;
        let query_string = match query_string.split_once('?') {
            Some(v) => v.1,
            None => return Err(Error::Referer),
        };
        let query_args = qs::from_str(query_string).map_err(|_| Error::InvalidQueryString)?;
        Ok(Params { server, query_args })
    }
}

#[derive(Deserialize, Debug)]
struct CustomScore {
    name: String,
    member: Option<String>,
    score_m: u8,
    score_cm: u8,
}

impl From<CustomScore> for Score {
    fn from(value: CustomScore) -> Self {
        let player_name = format!(
            "{}{}",
            if let Some(member) = value.member {
                format!("{member} ")
            } else {
                "".to_string()
            },
            value.name
        );
        let player_score = value.score_m as f64 + value.score_cm as f64 * 0.01;
        Self {
            player_name,
            player_score,
        }
    }
}

fn submit_score(params: &Params, score: Score) -> Result<reqwest::blocking::Response, Error> {
    let url = format!(
        "https://{}/{}/{}/{}",
        params.server, params.query_args.user, params.query_args.tournament, params.query_args.hole
    );
    let client = reqwest::blocking::Client::new();
    client.post(url).json(&score).send().map_err(|_| Error::Network)
}
