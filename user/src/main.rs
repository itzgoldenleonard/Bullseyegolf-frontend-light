#![recursion_limit = "512"]
mod get;
mod post;
use std::env;

/** 
 * Params That need to be collected externally for get
 * - Server
 * - Username            (qs)
 * - Tournament ID       (qs)
 * - Hole number         (qs)
 * - Show submit dialog? (qs)
 * - URL
 *
 * Params That need to be collected externally for post
 * - Server
 * - Username            (qs)
 * - Tournament ID       (qs)
 * - Hole number         (qs)
 */

fn main() {
    if env::var("REQUEST_METHOD") == Ok("POST".to_owned()) {
        post::post();
    } else {
        get::get();
    }
}

