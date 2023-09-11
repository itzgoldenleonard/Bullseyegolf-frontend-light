use html::root::builders::BodyBuilder;
use html::root::Html;
use serde::Deserialize;
use serde_qs as qs;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

enum Page {
    SelectTournament,
    SelectHole,
    ViewHole,
    SubmitScore,
}

type PageRenderer = Box<dyn for<'a> FnOnce(&'a mut BodyBuilder) -> &'a mut BodyBuilder>;

impl TryFrom<QueryParams> for Page {
    type Error = ();

    fn try_from(value: QueryParams) -> Result<Self, Self::Error> {
        use Page::*;

        Ok(match value.tournament {
            None => SelectTournament,
            Some(_) => match value.hole {
                None => SelectHole,
                Some(_) if value.submit == Some(true) => SubmitScore,
                Some(_) => ViewHole,
            },
        })
    }
}

struct Params {
    server: String,
    query_args: QueryParams,
    url: String,
}

#[derive(Debug, Deserialize, Clone)]
struct QueryParams {
    #[serde(rename = "u")]
    user: String,
    #[serde(rename = "t")]
    tournament: Option<String>,
    #[serde(rename = "h")]
    hole: Option<u8>,
    submit: Option<bool>,
}

impl Params {
    pub fn new() -> Self {
        let query_string = env::var("QUERY_STRING").unwrap();
        let query_args: QueryParams = qs::from_str(&query_string).unwrap();
        let server = env::var("SERVER_URL").unwrap();
        let url = env::var("REQUEST_URI").unwrap();
        Params {
            server,
            query_args,
            url,
        }
    }
}

/// Main entrypoint for the user interface (not the submit endpoint)
pub fn get() {
    let params: Params = Params::new();
    let page: Page = params.query_args.clone().try_into().unwrap();
    let content = page.find_page_function()(params);
    println!("{}", insert_into_template(content));
}

fn insert_into_template(content: PageRenderer) -> Html {
    Html::builder()
        .lang("da")
        .head(|head| {
            head.meta(|meta| meta.charset("utf-8"))
                .meta(|meta| {
                    meta.name("viewport")
                        .content("width=device-width, initial-scale=1")
                })
                .meta(|meta| meta.name("color-scheme").content("light dark"))
                .link(|link| link.rel("stylesheet").href("/user.css"))
                .title_attr("Bullseyegolf light")
        })
        .body(content)
        .build()
}

impl Page {
    pub fn find_page_function(self) -> fn(Params) -> PageRenderer {
        match self {
            Self::SelectTournament => Self::select_tournament_page,
            Self::SelectHole => Self::select_hole_page,
            Self::ViewHole => Self::view_hole_page,
            Self::SubmitScore => Self::submit_score_page,
        }
    }

    fn select_tournament_page(params: Params) -> PageRenderer {
        Box::new(move |b: &mut BodyBuilder| {
            let tournaments = fetch_short_tournaments(params.server, params.query_args.user);
            let active_tournaments = tournaments.iter().filter(|t| t.active);
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let recently_ended_tournaments: Vec<&ShortTournament> = tournaments
                .iter()
                .filter(|t| t.active == false && t.t_end >= current_time - 86400 * 3)
                .collect();
            let b = b
                .heading_1(|h1| h1.id("title").text("Vælg en turnering"))
                .heading_2(|h2| h2.text("Aktive turneringer"))
                .unordered_list(|ul| {
                    for tournament in active_tournaments {
                        ul.list_item(|li| {
                            li.anchor(|a| {
                                a.text(format!("{}", tournament.tournament_name))
                                    .href(format!("{}&t={}", params.url, tournament.tournament_id))
                            })
                        });
                    }
                    ul
                });
            if recently_ended_tournaments.len() == 0 {
                return b;
            } else {
                return b
                    .heading_2(|h2| h2.text("Afsluttede turneringer"))
                    .unordered_list(|ul| {
                        for tournament in recently_ended_tournaments {
                            ul.list_item(|li| {
                                li.anchor(|a| {
                                    a.text(format!("{}", tournament.tournament_name))
                                        .href(format!(
                                            "{}&t={}",
                                            params.url, tournament.tournament_id
                                        ))
                                })
                            });
                        }
                        ul
                    });
            }
        })
    }

    fn select_hole_page(params: Params) -> PageRenderer {
        Box::new(move |b: &mut BodyBuilder| {
            let tournament = fetch_tournament(
                params.server,
                params.query_args.user,
                params.query_args.tournament.unwrap(),
            );
            b.heading_1(|h1| h1.id("title").text(tournament.tournament_name))
                .paragraph(|p| p.text(format!("Sponsoreret af: {}", tournament.tournament_sponsor)))
                .heading_2(|h2| h2.text("Vælg et hul"))
                .unordered_list(|ul| {
                    for hole in tournament.holes {
                        ul.list_item(|li| {
                            li.anchor(|a| {
                                a.text(format!("Hul {}", hole.hole_number))
                                    .href(format!("{}&h={}", params.url, hole.hole_number))
                            })
                        });
                    }
                    ul
                })
        })
    }

    fn view_hole_page(_params: Params) -> PageRenderer {
        Box::new(|b: &mut BodyBuilder| b.paragraph(|p| p.text("This is the hole you selected")))
    }

    fn submit_score_page(_params: Params) -> PageRenderer {
        Box::new(|b: &mut BodyBuilder| b.paragraph(|p| p.text("Submit your score here")))
    }
}

#[derive(Deserialize)]
struct ShortTournament {
    active: bool,
    //t_start: u64,
    t_end: u64,
    tournament_id: String,
    tournament_name: String,
}

// TODO: Make this a fallible function
fn fetch_short_tournaments(server: String, username: String) -> Vec<ShortTournament> {
    let url = format!("https://{server}/{username}");
    reqwest::blocking::get(url).unwrap().json().unwrap()
}

#[derive(Deserialize, Debug)]
struct Tournament {
    tournament_name: String,
    tournament_sponsor: String,
    holes: Vec<Hole>,
}

// TODO: Make this a fallible function
fn fetch_tournament(server: String, username: String, tournament_id: String) -> Tournament {
    let url = format!("https://{server}/{username}/{tournament_id}");
    let client = reqwest::blocking::Client::new();
    client
        .get(url)
        .header("No-Hole-Images", "true")
        .send()
        .unwrap()
        .json()
        .unwrap()
}

#[derive(Deserialize, Debug)]
struct Hole {
    hole_number: u8,
    hole_text: String,
    hole_sponsor: String,
    scores: Vec<Score>,
}

#[derive(Deserialize, Debug)]
struct Score {
    player_name: String,
    player_score: f64,
}
