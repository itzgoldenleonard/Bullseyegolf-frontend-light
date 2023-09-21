use crate::error::Error;
use html::inline_text::Anchor;
use html::root::children::BodyChild;
use html::root::{Body, Html};
use html::tables::{TableBody, TableRow};
use html::text_content::{Paragraph, UnorderedList};
use serde::{Deserialize, Serialize};
use serde_urlencoded as qs;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use tuple::Map;

/// Main entrypoint for the user interface (not the submit endpoint)
pub fn get() -> Result<String, Error> {
    let params: Params = Params::new()?;
    let content = params.try_into()?;
    Ok(insert_into_template(content).to_string())
}

struct Params {
    server: String,
    query_args: QueryParams,
}

#[derive(Deserialize)]
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
            env::var("QUERY_STRING").map_err(|e| Error::EnvVarRead("QUERY_STRING", e))?;
        let query_args = qs::from_str(&query_string).map_err(|_| Error::InvalidQueryString)?;
        let server = env::var("SERVER_URL").map_err(|e| Error::EnvVarRead("SERVER_URL", e))?;
        Ok(Params { server, query_args })
    }
}

impl TryFrom<Params> for Body {
    type Error = Error;

    fn try_from(value: Params) -> Result<Self, Self::Error> {
        let user = value.query_args.user;
        let tournament = value.query_args.tournament;
        let hole = value.query_args.hole;
        let server = value.server;

        match tournament {
            None => SelectTournamentPage { user }.render(&server),
            Some(tournament) => match hole {
                None => SelectHolePage { user, tournament }.render(&server),
                Some(hole) => ViewHolePage {
                    user,
                    tournament,
                    hole,
                }
                .render(&server),
            },
        }
    }
}

struct SelectTournamentPage {
    user: String,
}

impl Render for SelectTournamentPage {
    fn render(&self, server: &str) -> Result<Body, Error> {
        let tournaments: Vec<ShortTournament> = Fetch::fetch(server, self)?;
        let current_time = secs_since_epoch()?;

        let create_ul = |c: Vec<ShortTournament>| {
            c.iter()
                .map(|t| {
                    Anchor::builder()
                        .text(t.tournament_name.to_string())
                        .href(format!("?u={}&t={}", self.user, t.tournament_id))
                        .build()
                })
                .fold(&mut UnorderedList::builder(), |acc, a| {
                    acc.list_item(|li| li.push(a))
                })
                .build()
        }; // TODO: This might make sense as a trait

        let (active, inactive) = tournaments
            .into_iter()
            .filter(|t| t.t_end >= current_time - 86400 * 3 && t.t_start < current_time)
            .partition(|t| t.active)
            .map(create_ul);

        let mut b = Body::builder();
        b.heading_1(|h1| h1.id("title").text("Vælg en turnering"))
            .heading_2(|h2| h2.text("Aktive turneringer"))
            .push(if !active.children().is_empty() {
                BodyChild::UnorderedList(active)
            } else {
                BodyChild::Paragraph(
                    Paragraph::builder()
                        .text("Ingen aktive turneringer")
                        .build(),
                )
            });

        if !inactive.children().is_empty() {
            b.heading_2(|h2| h2.text("Afsluttede turneringer"))
                .push(inactive);
        };

        Ok(b.build())
    }
}

fn secs_since_epoch() -> Result<u64, Error> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| {
            Error::GenericServer("Unable to calculate current time (time went backwards)")
        })?
        .as_secs())
}

struct SelectHolePage {
    user: String,
    tournament: String,
}

impl Render for SelectHolePage {
    fn render(&self, server: &str) -> Result<Body, Error> {
        let tournament = Tournament::fetch(server, self)?;
        let holes = tournament
            .holes
            .iter()
            .map(|h| {
                Anchor::builder()
                    .text(format!("Hul {}", h.hole_number))
                    .href(format!(
                        "?u={}&t={}&h={}",
                        self.user, self.tournament, h.hole_number
                    ))
                    .build()
            })
            .fold(&mut UnorderedList::builder(), |acc, a| {
                acc.list_item(|li| li.push(a))
            })
            .build();

        let mut b = Body::builder();
        b.heading_1(|h1| h1.id("title").text(tournament.tournament_name));

        if !tournament.tournament_sponsor.is_empty() {
            b.paragraph(|p| p.text(format!("Sponsoreret af: {}", tournament.tournament_sponsor)));
        };

        b.heading_2(|h2| h2.text("Vælg et hul"))
            .push(if !holes.children().is_empty() {
                BodyChild::UnorderedList(holes)
            } else {
                BodyChild::Paragraph(
                    Paragraph::builder()
                        .text("Der er ingen huller i denne turnering")
                        .build(),
                )
            });

        Ok(b.build())
    }
}

struct ViewHolePage {
    user: String,
    tournament: String,
    hole: u8,
}

impl Render for ViewHolePage {
    fn render(&self, server: &str) -> Result<Body, Error> {
        let hole = Hole::fetch(server, self)?;
        let hole_text = if !hole.hole_text.is_empty() {
            hole.hole_text
        } else {
            format!("Hul {}", hole.hole_number)
        };
        let no_scores = hole.scores.is_empty();
        let scores = hole.scores.into_iter().enumerate().map(|s| {
            TableRow::builder()
                .table_cell(|td| td.text(format!("{}.", s.0 + 1)))
                .table_cell(|td| td.text(s.1.player_name.clone()))
                .table_cell(|td| {
                    td.text(format!("{:.2}m", s.1.player_score,).replacen(".", ",", 1))
                })
                .build()
        });

        let mut scores_builder = TableBody::builder();
        if no_scores {
            scores_builder.table_row(|tr| {
                tr.table_cell(|td| td.text("Der er ingen noteringer endnu").colspan("3"))
            });
        } else {
            scores_builder.extend(scores);
        };

        let mut b = Body::builder();
        b.heading_1(|h1| h1.id("title").text(hole_text));

        if !hole.hole_sponsor.is_empty() {
            b.paragraph(|p| p.text(format!("Sponsoreret af: {}", hole.hole_sponsor)));
        };

        b.table(|table| {
            table
                .table_head(|thead| {
                    thead.table_row(|tr| {
                        tr.table_header(|th| th.text("Nr.").scope("col"))
                            .table_header(|th| th.text("Navn").scope("col"))
                            .table_header(|th| th.text("Score").scope("col"))
                    })
                })
                .push(scores_builder.build());
            table
        });

        if self.active(server)? {
            b.anchor(|a| {
                a.href(format!(
                    "/submit_score.html?u={}&t={}&h={}",
                    self.user, self.tournament, hole.hole_number
                ))
                .text("Indsend notering")
            });
        };

        Ok(b.build())
    }
}

impl From<&ViewHolePage> for SelectTournamentPage {
    fn from(value: &ViewHolePage) -> Self {
        Self {
            user: value.user.clone(),
        }
    }
}

impl ViewHolePage {
    fn active(&self, server: &str) -> Result<bool, Error> {
        let tournament_list: Vec<ShortTournament> = Fetch::fetch(server, &self.into())?;
        Ok(tournament_list
            .into_iter()
            .find(|t| t.tournament_id == self.tournament)
            .map(|t| t.active)
            .unwrap_or(false))
    }
}

#[derive(Deserialize)]
struct ShortTournament {
    active: bool,
    t_start: u64,
    t_end: u64,
    tournament_id: String,
    tournament_name: String,
}

impl Fetch for Vec<ShortTournament> {
    type Page = SelectTournamentPage;

    fn fetch(server: &str, page: &Self::Page) -> Result<Self, Error> {
        let url = format!("https://{}/{}", server, page.user);
        reqwest::blocking::get(url)
            .map_err(|_| Error::Network)?
            .json()
            .map_err(|_| Error::Network)
    }
}

#[derive(Deserialize)]
struct Tournament {
    //tournament_id: String,
    tournament_name: String,
    tournament_sponsor: String, // Optional
    holes: Vec<Hole>,           // Optional
}

impl Fetch for Tournament {
    type Page = SelectHolePage;

    fn fetch(server: &str, page: &Self::Page) -> Result<Self, Error> {
        let url = format!("https://{server}/{}/{}", page.user, page.tournament);
        let client = reqwest::blocking::Client::new();
        client
            .get(url)
            .header("No-Hole-Images", "true")
            .send()
            .map_err(|_| Error::Network)?
            .json()
            .map_err(|_| Error::Network)
    }
}

#[derive(Deserialize)]
struct Hole {
    hole_number: u8,
    hole_text: String,    // Optional
    hole_sponsor: String, // Optional
    scores: Vec<Score>,   // Optional
}

#[derive(Deserialize, Serialize)]
pub struct Score {
    pub player_name: String,
    pub player_score: f64,
}

impl Fetch for Hole {
    type Page = ViewHolePage;

    fn fetch(server: &str, page: &Self::Page) -> Result<Self, Error> {
        let url = format!(
            "https://{server}/{}/{}/{}",
            page.user, page.tournament, page.hole
        );
        reqwest::blocking::get(url)
            .map_err(|_| Error::Network)?
            .json()
            .map_err(|_| Error::Network)
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

trait Render {
    fn render(&self, server: &str) -> Result<Body, Error>;
}

trait Fetch: Sized {
    type Page;
    fn fetch(server: &str, page: &Self::Page) -> Result<Self, Error>;
}
