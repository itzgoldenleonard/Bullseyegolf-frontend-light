use crate::error::Error;
use html::root::{Body, Html};
use serde::{Deserialize, Serialize};
use serde_urlencoded as qs;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

/// Main entrypoint for the user interface (not the submit endpoint)
pub fn get() -> Result<String, Error> {
    let params: Params = Params::new()?;
    let page: Page = params.query_args.into();
    let content = page.find_page_function(&params.server)?;
    Ok(insert_into_template(content).to_string())
}

struct Params {
    server: String,
    query_args: QueryParams,
}

#[derive(Debug, Deserialize, Clone)]
struct QueryParams {
    #[serde(rename = "u")]
    user: String,
    #[serde(rename = "t")]
    tournament: Option<String>,
    #[serde(rename = "h")]
    hole: Option<u8>,
}

impl Params {
    pub fn new() -> Result<Self, Error> {
        let query_string =
            env::var("QUERY_STRING").map_err(|e| Error::EnvVarReadError("QUERY_STRING", e))?;
        let query_args = qs::from_str(&query_string).map_err(|_| Error::InvalidQueryString)?;
        let server = env::var("SERVER_URL").map_err(|e| Error::EnvVarReadError("SERVER_URL", e))?;
        Ok(Params { server, query_args })
    }
}

enum Page {
    SelectTournament{user: String},
    SelectHole{user: String, tournament: String},
    ViewHole{user: String, tournament: String, hole: u8},
}

impl From<QueryParams> for Page {
    fn from(value: QueryParams) -> Self {
        use Page::*;
        match value.tournament {
            None => SelectTournament{user: value.user},
            Some(tournament) => match value.hole {
                None => SelectHole{user: value.user, tournament},
                Some(hole) => ViewHole{user: value.user, tournament, hole},
            },
        }
    }
}

impl Page {
    pub fn find_page_function(self, server: &str) -> Result<Body, Error> {
        match self {
            Self::SelectTournament{user} => Self::select_tournament_page(server, &user),
            Self::SelectHole{user, tournament} => Self::select_hole_page(server, &user, &tournament),
            Self::ViewHole{user, tournament, hole} => Self::view_hole_page(server, &user, &tournament, &hole),
        }
    }

    fn select_tournament_page(server: &str, user: &str) -> Result<Body, Error> {
        let tournaments = fetch_short_tournaments(server, user)?;
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| {
                Error::GenericServerError("Unable to calculate current time (time went backwards)")
            })?
            .as_secs();

        let mut b = Body::builder();

        let active_tournaments = tournaments.iter().filter(|t| t.active);
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
                                .href(format!(
                                    "?u={}&t={}",
                                    user, tournament.tournament_id
                                ))
                        })
                    });
                }
                ul
            });
        if recently_ended_tournaments.len() == 0 {
            return Ok(b.build());
        } else {
            return Ok(b
                .heading_2(|h2| h2.text("Afsluttede turneringer"))
                .unordered_list(|ul| {
                    for tournament in recently_ended_tournaments {
                        ul.list_item(|li| {
                            li.anchor(|a| {
                                a.text(format!("{}", tournament.tournament_name))
                                    .href(format!(
                                        "?u={}&t={}",
                                        user, tournament.tournament_id
                                    ))
                            })
                        });
                    }
                    ul
                })
                .build());
        }
    }

    fn select_hole_page(server: &str, user: &str, tournament: &str) -> Result<Body, Error> {
        let tournament = fetch_tournament(
            server,
            user,
            tournament,
        )?;

        let mut b = Body::builder();

        Ok(
            b.heading_1(|h1| h1.id("title").text(tournament.tournament_name))
                .paragraph(|p| p.text(format!("Sponsoreret af: {}", tournament.tournament_sponsor)))
                .heading_2(|h2| h2.text("Vælg et hul"))
                .unordered_list(|ul| {
                    for hole in tournament.holes {
                        ul.list_item(|li| {
                            li.anchor(|a| {
                                a.text(format!("Hul {}", hole.hole_number)).href(format!(
                                    "?u={}&t={}&h={}",
                                    user,
                                    tournament.tournament_id,
                                    hole.hole_number
                                ))
                            })
                        });
                    }
                    ul
                })
                .build(),
        )
    }

    fn view_hole_page(server: &str, user: &str, tournament: &str, hole: &u8) -> Result<Body, Error> {
        let hole = fetch_hole(
            server,
            user,
            tournament,
            hole,
        )?;

        let mut b = Body::builder();

        Ok(b.heading_1(|h1| h1.id("title").text(hole.hole_text))
            .paragraph(|p| p.text(format!("Sponsoreret af: {}", hole.hole_sponsor)))
            .table(|table| {
                table
                    .table_head(|thead| {
                        thead.table_row(|tr| {
                            tr.table_header(|th| th.text("Nr.").scope("col"))
                                .table_header(|th| th.text("Navn").scope("col"))
                                .table_header(|th| th.text("Score").scope("col"))
                        })
                    })
                    .table_body(|tbody| {
                        for score in hole.scores.iter().enumerate() {
                            tbody.table_row(|tr| {
                                tr.table_cell(|td| td.text(format!("{}.", score.0 + 1)))
                                    .table_cell(|td| td.text(score.1.player_name.clone()))
                                    .table_cell(|td| td.text(format!("{}m", score.1.player_score)))
                            });
                        }
                        tbody
                    });
                table
            })
            .anchor(|a| {
                a.href(format!(
                    "/submit_score.html?u={}&t={}&h={}",
                    user,
                    tournament,
                    hole.hole_number
                ))
                .text("Indsend notering")
            })
            .build())
    }
}

fn insert_into_template(content: Body) -> Html {
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
        .push(content)
        .build()
}

#[derive(Deserialize)]
struct ShortTournament {
    active: bool,
    //t_start: u64,
    t_end: u64,
    tournament_id: String,
    tournament_name: String,
}

fn fetch_short_tournaments(server: &str, username: &str) -> Result<Vec<ShortTournament>, Error> {
    let url = format!("https://{server}/{username}");
    reqwest::blocking::get(url)
        .map_err(|_| Error::NetworkError)?
        .json()
        .map_err(|_| Error::NetworkError)
}

#[derive(Deserialize, Debug)]
struct Tournament {
    tournament_id: String,
    tournament_name: String,
    tournament_sponsor: String,
    holes: Vec<Hole>,
}

fn fetch_tournament(
    server: &str,
    username: &str,
    tournament_id: &str,
) -> Result<Tournament, Error> {
    let url = format!("https://{server}/{username}/{tournament_id}");
    let client = reqwest::blocking::Client::new();
    client
        .get(url)
        .header("No-Hole-Images", "true")
        .send()
        .map_err(|_| Error::NetworkError)?
        .json()
        .map_err(|_| Error::NetworkError)
}

// Make a function that builds a url from QueryParams

#[derive(Deserialize, Debug)]
struct Hole {
    hole_number: u8,
    hole_text: String,
    hole_sponsor: String,
    scores: Vec<Score>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Score {
    pub player_name: String,
    pub player_score: f64,
}

fn fetch_hole(
    server: &str,
    username: &str,
    tournament_id: &str,
    hole_number: &u8,
) -> Result<Hole, Error> {
    let url = format!("https://{server}/{username}/{tournament_id}/{hole_number}");
    reqwest::blocking::get(url)
        .map_err(|_| Error::NetworkError)?
        .json()
        .map_err(|_| Error::NetworkError)
}
