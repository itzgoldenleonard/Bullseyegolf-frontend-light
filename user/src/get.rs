use std::env;
use serde::Deserialize;
use serde_qs as qs;
use super::ShortTournament;
use html::content::Main;

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

/// Main entrypoint for the user interface (not the submit endpoint)
pub fn get() {
    let query_string = env::var("QUERY_STRING").unwrap();
    let params: Params = qs::from_str(&query_string).unwrap();
    let content = generate_content(params);
    println!("{content}");
}

/// This function decides which page to display
fn generate_content(params: Params) -> Main {
    match params.tournament {
        None => select_tournament_page(params),
        Some(_) => match params.hole {
            None => select_hole_page(params),
            Some(_) => view_hole_page(params),
        }
    }
}

fn select_tournament_page(params: Params) -> Main {
    Main::builder().unordered_list(|list| {
        let url = format!("https://{}/{}", params.server, params.user);
        println!("Select a tournament from: {url}");
        let request: Vec<ShortTournament> = reqwest::blocking::get(url).unwrap().json().unwrap();
        let active_tournaments = request.iter().filter(|t| t.active);
        for tournament in active_tournaments {
            list.list_item(|li| li.text(format!("{}", tournament.tournament_name)));
        }
        list
    }).build()
}

fn select_hole_page(_params: Params) -> Main {
    Main::builder().paragraph(|p| p.text("Select a hole")).build()
}

fn view_hole_page(_params: Params) -> Main {
    Main::builder().paragraph(|p| p.text("This is the hole you selected")).build()
}
