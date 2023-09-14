use crate::get::Score;
use serde::Deserialize;
use serde_urlencoded as qs;
use std::env;
use std::io::stdin;

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
        Params { server, query_args }
    }
}

/// Main entrypoint for score submission
pub fn post() {
    let params = Params::new();

    let mut score_buffer = String::new();
    let _ = stdin().read_line(&mut score_buffer);
    let score: CustomScore = qs::from_str(&score_buffer).unwrap(); // TODO: Try with from_reader
    submit_score(&params, score.into());
    println!("Status: 303\r");
    println!("Location: ?u={}&t={}&h={}\r", params.query_args.user, params.query_args.tournament, params.query_args.hole);
    println!("\r\n\r");
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

fn submit_score(params: &Params, score: Score) {
    let url = format!(
        "https://{}/{}/{}/{}",
        params.server, params.query_args.user, params.query_args.tournament, params.query_args.hole
    );
    let client = reqwest::blocking::Client::new();
    let _res = client.post(url).json(&score).send();
}
