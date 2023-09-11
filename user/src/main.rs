#![recursion_limit = "512"]
mod get;

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
    get::get();
}

