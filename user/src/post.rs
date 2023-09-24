use crate::error::Error;
use crate::get::{Fetch, Hole, Score, ViewHolePage};
use serde::Deserialize;
use serde_urlencoded as qs;
use std::env;
use std::io::stdin;

/// Main entrypoint for score submission
pub fn post() -> Result<String, Error> {
    let params = Params::new()?;
    let score: Score = qs::from_reader::<CustomScore, _>(stdin())
        .map_err(|_| Error::InvalidForm)?
        .into();

    let leaderboard = Hole::fetch(&params.server, &params.query_args)?.scores;
    if !score.is_duplicate(&leaderboard) {
        submit_score(&params, score, &leaderboard)?;
    }

    Ok(format!(
        "Status: 303\r\nLocation: ?u={}&t={}&h={}\r\n\r\n\r\n",
        params.query_args.user, params.query_args.tournament, params.query_args.hole
    ))
}

struct Params {
    server: String,
    query_args: ViewHolePage,
}

impl Params {
    pub fn new() -> Result<Self, Error> {
        let server = env::var("SERVER_URL")?;
        let query_string = env::var("HTTP_REFERER").map_err(|_| Error::Referer)?;
        let query_string = query_string.split_once('?').ok_or(Error::Referer)?.1;
        let query_args = qs::from_str(query_string)?;
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
            value.member.map(|m| m + " ").unwrap_or_default(),
            value.name
        );
        Self {
            player_name,
            player_score: value.score_m as f64 + value.score_cm as f64 * 0.01,
        }
    }
}

impl Score {
    fn is_first(&self, leaderboard: &[Score]) -> bool {
        if leaderboard.is_empty() {
            return true;
        };
        self.player_score < leaderboard[0].player_score
    }

    fn is_duplicate(&self, leaderboard: &[Score]) -> bool {
        leaderboard.iter().any(|s| s == self)
    }
}

fn submit_score(
    params: &Params,
    mut score: Score,
    leaderboard: &[Score],
) -> Result<reqwest::blocking::Response, Error> {
    if !score.is_first(leaderboard) {
        score.player_name += " 🏴";
    }

    let url = format!(
        "{}/{}/{}/{}",
        params.server, params.query_args.user, params.query_args.tournament, params.query_args.hole
    );
    let client = reqwest::blocking::Client::new();
    client
        .post(url)
        .json(&score)
        .send()
        .map_err(|_| Error::Network)
}
