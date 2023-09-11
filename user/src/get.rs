use serde::Deserialize;
use serde_qs as qs;
use std::env;
//use super::ShortTournament;
use html::root::builders::BodyBuilder;
use html::root::Html;

enum Page {
    SelectTournament,
    SelectHole,
    ViewHole,
    SubmitScore,
}

type PageRenderer = Box<dyn for<'a> FnOnce(&'a mut BodyBuilder) -> &'a mut BodyBuilder>;

impl TryFrom<Params> for Page {
    type Error = ();

    fn try_from(value: Params) -> Result<Self, Self::Error> {
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

#[derive(Debug, Deserialize, Clone)]
struct Params {
    #[serde(rename = "s")]
    server: String,
    #[serde(rename = "u")]
    user: String,
    #[serde(rename = "t")]
    tournament: Option<String>,
    #[serde(rename = "h")]
    hole: Option<u8>,
    submit: Option<bool>,
}

/// Main entrypoint for the user interface (not the submit endpoint)
pub fn get() {
    let query_string = env::var("QUERY_STRING").unwrap();
    let params: Params = qs::from_str(&query_string).unwrap();
    let page: Page = params.clone().try_into().unwrap();
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

    fn select_tournament_page(_params: Params) -> PageRenderer {
        Box::new(|b: &mut BodyBuilder| b.paragraph(|p| p.text("Select a tournament")))
    }

    fn select_hole_page(_params: Params) -> PageRenderer {
        Box::new(|b: &mut BodyBuilder| b.paragraph(|p| p.text("Select a hole")))
    }

    fn view_hole_page(_params: Params) -> PageRenderer {
        Box::new(|b: &mut BodyBuilder| b.paragraph(|p| p.text("This is the hole you selected")))
    }

    fn submit_score_page(_params: Params) -> PageRenderer {
        Box::new(|b: &mut BodyBuilder| b.paragraph(|p| p.text("Submit your score here")))
    }
}

/*
let url = format!("https://{}/{}", params.server, params.user);
println!("Select a tournament from: {url}");
let request: Vec<ShortTournament> = reqwest::blocking::get(url).unwrap().json().unwrap();
let active_tournaments = request.iter().filter(|t| t.active);
for tournament in active_tournaments {
    list.list_item(|li| li.text(format!("{}", tournament.tournament_name)));
}
list
*/
