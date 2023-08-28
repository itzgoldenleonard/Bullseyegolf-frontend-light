#![recursion_limit = "512"]
use std::env;
use serde::Deserialize;
use serde_qs as qs;
use html::text_content::UnorderedList;

#[derive(Debug, Deserialize)]
struct Params {
    #[serde(rename = "s")]
    server: String,
    #[serde(rename = "u")]
    user: String,
    #[serde(rename = "t")]
    tournament: Option<String>,
    #[serde(rename = "h")]
    hole: Option<u8>,
}

#[derive(Debug, Deserialize)]
struct ShortTournament {
    active: bool,
    t_start: u64,
    t_end: u64,
    tournament_id: String,
    tournament_name: String,
}


fn main() {
    let query_string = env::var("QUERY_STRING").unwrap();
    let deserialized: Params = qs::from_str(&query_string).unwrap();
    match deserialized.tournament {
        None => select_tournament_page(deserialized),
        Some(_) => match deserialized.hole {
            None => select_hole_page(deserialized),
            Some(_) => view_hole_page(deserialized),
        }
    }
}

fn select_tournament_page(params: Params) {
    let url = format!("https://{}/{}", params.server, params.user);
    println!("Select a tournament from: {url}");
    let request: Vec<ShortTournament> = reqwest::blocking::get(url).unwrap().json().unwrap();
    let active_tournaments = request.iter().filter(|t| t.active);
    let mut list = UnorderedList::builder();
    for tournament in active_tournaments {
        list.list_item(|li| li.text(format!("{}", tournament.tournament_name)));
    }
    println!("{}", list.build());
}

fn select_hole_page(params: Params) {
    println!("Select a hole");
}

fn view_hole_page(params: Params) {
    println!("This is the hole you selected");
}
