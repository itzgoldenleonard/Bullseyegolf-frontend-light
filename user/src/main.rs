#![recursion_limit = "512"]
mod error;
mod get;
mod post;
use std::env;

fn main() {
    let response = if env::var("REQUEST_METHOD") == Ok("POST".to_owned()) {
        post::post()
    } else {
        get::get()
    };
    match response {
        Ok(t) => println!("{t}"),
        Err(e) => println!("{e}"),
    }
}
