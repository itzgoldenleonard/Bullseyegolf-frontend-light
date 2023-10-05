#![recursion_limit = "512"]
/// Error handling for the entire program
mod error;
/// Generates the requested page
mod get;
/// Forwards the score submission to the API server and redirects to [`get::ViewHolePage`]
mod post;
use std::env;

/**
 * Main entrypoint for the program
 *
 * Calls the correct module depending on the HTTP method used.
 * Also handles errors by printing them to stdout
 */
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
