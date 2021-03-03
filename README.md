# Intro

This mini project has been created as a part of my effort to understand performance of Rust's `HashMap` inserts when having `&str` keys and `String` keys. The goal is to pick a way to construct and use an in-memory cache.

Here's a link to a [relevant Rust Users forum thread](https://users.rust-lang.org/t/am-i-doing-str-copying-into-a-long-lived-map-correctly/56346).

# How to run the benchmarks

- Clone this repo;
- `cd` into it;
- run `cargo bench`.

# Last known benchmark results

On a Xeon-2150B CPU:

```plain
insert_str              time:   [1.0253 us 1.0409 us 1.0561 us]
insert_string           time:   [5.8288 us 5.8547 us 5.8786 us]
```

# Conclusion

Even though inserting map entries with `&str` seems to be ~5.8x faster it's usually a headache to manage lifetimes and ownership. And since the map has to own the keys anyway it's just more ergonomic to use `String`s for map keys -- their ownership transfer is intuitive and has no hidden surprises as `&str` keys could have (f.ex. if they are coming from parsed file the compiler won't let you release them because you are now also referencing them inside a map; this proved to be too annoying to manage manually).

Thus, even with the given performance differences and for the goal of having a cache that's 99% reads and 1% writes/deletes (which is my use-case), this performance difference is not significant.
