#![recursion_limit = "512"]
mod get;

use serde::Deserialize;

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

