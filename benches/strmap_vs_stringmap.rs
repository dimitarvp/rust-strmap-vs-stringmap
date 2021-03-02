use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

fn insert_str<'a>(map: &mut HashMap<&'a str, usize>, key: &'a str) {
    map.insert(key, 0);
}

fn insert_string<T: Into<String>>(map: &mut HashMap<String, usize>, key: T) {
    map.insert(key.into(), 0);
}

fn test_insert_str() {
    let k0 = "$HOME/1.file";
    let k1 = "~/2.file";
    let k2 = "/Users/somebody/3.file";
    let k3 = String::from("/home/somebody/someplace/4.file");
    let k4 = String::from("/path/to/somewhere/5.file");
    let mut map = HashMap::<&str, usize>::new();
    insert_str(&mut map, k0);
    insert_str(&mut map, k1);
    insert_str(&mut map, k2);
    insert_str(&mut map, &k3);
    insert_str(&mut map, &k4);
}

fn test_insert_string() {
    let k0 = "$HOME/1.file";
    let k1 = "~/2.file";
    let k2 = "/Users/somebody/3.file";
    let k3 = String::from("/home/somebody/someplace/4.file");
    let k4 = String::from("/path/to/somewhere/5.file");
    let mut map = HashMap::<String, usize>::new();
    insert_string(&mut map, k0);
    insert_string(&mut map, k1);
    insert_string(&mut map, k2);
    insert_string(&mut map, k3);
    insert_string(&mut map, k4);
}

fn benchmark_insert_str(c: &mut Criterion) {
    c.bench_function("insert_str", |b| b.iter(|| test_insert_str()));
}

fn benchmark_insert_string(c: &mut Criterion) {
    c.bench_function("insert_string", |b| b.iter(|| test_insert_string()));
}

criterion_group!(benches, benchmark_insert_str, benchmark_insert_string);
criterion_main!(benches);
