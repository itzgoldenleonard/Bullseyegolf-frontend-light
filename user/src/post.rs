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
        .map_err(Error::InvalidForm)?
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

/**
 * All the state needed to know how to run the program
 *
 * This is different from [`crate::get`]'s `Params` because none of the `query_args` are optional
 */
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

/**
 * The custom format for a score used by the HTML form
 *
 * Can be losslessly converted to a proper [`Score`]
 */
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
        leaderboard
            .get(0)
            .map(|s| self.player_score < s.player_score)
            .unwrap_or(true)
    }

    /// Does this Score already exist in the given leaderboard
    fn is_duplicate(&self, leaderboard: &[Score]) -> bool {
        leaderboard.iter().any(|s| s == self)
    }
}

/**
 * Submits `score` to the API server
 *
 * Only submits if `score` is not already present in `leaderboard`
 *
 * If `score` is not first in `leaderboard` the "🏴" emoji is appended to its name
 */
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
    Ok(client.post(url).json(&score).send()?.error_for_status()?)
}
