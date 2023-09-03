#![recursion_limit = "512"]
mod get;

use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct ShortTournament {
    active: bool,
    t_start: u64,
    t_end: u64,
    tournament_id: String,
    tournament_name: String,
}


fn main() {
    get::get();
}

