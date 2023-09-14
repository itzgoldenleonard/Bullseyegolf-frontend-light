use std::env;
use serde::Deserialize;
use crate::get::Score;
use serde_urlencoded as qs;

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
    // TODO: make this fallible
    pub fn new() -> Self {
        let server = env::var("SERVER_URL").unwrap();
        let query_string = env::var("HTTP_REFERER").unwrap();
        let query_string = query_string.split_once("?").unwrap().1;
        let query_args: QueryParams = qs::from_str(&query_string).unwrap();
        Params {
            server,
            query_args,
        }
    }
}

/// Main entrypoint for score submission
pub fn post() {
    let params = Params::new();
    let score = Score { player_name: String::from("Test"), player_score: 1.09 };
    submit_score(params, score);
    println!("Submitting your score");
}

fn submit_score(params: Params, score: Score) {
    let url = format!("https://{}/{}/{}/{}", params.server, params.query_args.user, params.query_args.tournament, params.query_args.hole);
    let client = reqwest::blocking::Client::new();
    let _res = client.post(url).json(&score).send();
}
