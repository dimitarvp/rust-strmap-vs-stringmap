# Intro

This mini project has been created as a part of my effort to understand performance of Rust's `HashMap` inserts, lookups and deletions when having `&str` keys and `String` keys.

Here's a link to a [relevant Rust Users forum thread](https://users.rust-lang.org/t/am-i-doing-str-copying-into-a-long-lived-map-correctly/56346).

# How to run the benchmarks

- Clone this repo;
- `cd` into it;
- run `cargo bench`.
