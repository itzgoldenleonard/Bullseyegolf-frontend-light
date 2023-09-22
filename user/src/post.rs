use crate::error::Error;
use crate::get::{Fetch, Hole, Score, ViewHolePage};
use serde::Deserialize;
use serde_urlencoded as qs;
use std::env;
use std::io::stdin;

/// Main entrypoint for score submission
pub fn post() -> Result<String, Error> {
    let params = Params::new()?;
    let score: CustomScore = qs::from_reader(stdin()).map_err(|_| Error::InvalidForm)?;

    let page = ViewHolePage {
        user: params.query_args.user.clone(),
        tournament: params.query_args.tournament.clone(),
        hole: params.query_args.hole,
    };
    let hole = Hole::fetch(&params.server, &page)?;

    submit_score(&params, score.into(), hole)?;
    Ok(format!(
        "Status: 303\r\nLocation: ?u={}&t={}&h={}\r\n\r\n\r\n",
        page.user, page.tournament, page.hole
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

impl Score {
    fn is_first(&self, leaderboard: &[Score]) -> bool {
        if leaderboard.is_empty() { return true };
        self.player_score < leaderboard[0].player_score
    }

    fn is_duplicate(&self, leaderboard: &[Score]) -> bool {
        leaderboard.iter().any(|s| s == self)
    }
}

fn submit_score(params: &Params, mut score: Score, leaderboard: Hole) -> Result<(), Error> {
    let url = format!(
        "{}/{}/{}/{}",
        params.server, params.query_args.user, params.query_args.tournament, params.query_args.hole
    );

    if !score.is_first(&leaderboard.scores) {
        score.player_name += " 🏴";
    }

    if !score.is_duplicate(&leaderboard.scores) {
        let client = reqwest::blocking::Client::new();
        client
            .post(url)
            .json(&score)
            .send()
            .map_err(|_| Error::Network)?;
    }

    Ok(())
}
