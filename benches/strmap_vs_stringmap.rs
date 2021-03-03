use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

static KEYS: [&str; 32] = [
    "/usr/olivia/etc/woman.csv",
    "/bin/week/company/var/mia/elijah.rar",
    "/var/hand/different/usr/group/company.ppt",
    "/var/same/logan/little/public.flv",
    "/home/lucas/sophia/large/able.doc",
    "/sbin/mason/high/next/company/usr/great/same/first.jpg",
    "/tmp/large/man/tmp/year/liam/mason.tar",
    "/bin/big/var/person.png",
    "/var/large/sophia/aria/jackson/sbin/james/important/number.doc",
    "/usr/sbin/same/government/first.csv",
    "/home/next/able/etc/oliver/man.csv",
    "//life.zip",
    "/var/lucas/different/usr/isabella/large.mp3",
    "/long/different/etc/way/lucas.so",
    "/sbin/part/number/usr/next.mp3",
    "/time/lucas/public/var/few/logan/old.rar",
    "/etc/way/eye/tmp/thing.jpg",
    "/etc/liam/bad/new/sbin/child/company.ppt",
    "/sbin/part/jackson/var/place/noah.xls",
    "/usr/little/eye/old/group/etc.ppt",
    "/var/high/work/home/group/important.rar",
    "/sbin/last/noah/long/sbin.zip",
    "/bin/company/var/case.txt",
    "/group/bin/james/public/emma.doc",
    "/var/liam/var/mason.ppt",
    "//usr/olivia/own.csv",
    "/etc/emma/year/home/life/man.zip",
    "/etc/last/home/aria/eye.zip",
    "/usr/elijah/home/point/few/week/place.rar",
    "/home/oliver/amelia/same/usr.doc",
    "/var/oliver/fact/elijah/sbin/small/emma/other/place.tar",
    "/home/woman/charlotte/sbin/own/case.mp4",
];

fn insert_str<'a>(map: &mut HashMap<&'a str, usize>, key: &'a str) {
    map.insert(key, 0);
}

fn insert_string<T: Into<String>>(map: &mut HashMap<String, usize>, key: T) {
    map.insert(key.into(), 0);
}

fn test_insert_str() {
    let mut map = HashMap::<&str, usize>::with_capacity(32);
    KEYS.iter().for_each(|k| insert_str(&mut map, k));
}

fn test_insert_string() {
    let mut map = HashMap::<String, usize>::with_capacity(32);
    KEYS.iter()
        .for_each(|k| insert_string(&mut map, k.to_string()));
}

fn benchmark_insert_str(c: &mut Criterion) {
    c.bench_function("insert_str", |b| b.iter(|| test_insert_str()));
}

fn benchmark_insert_string(c: &mut Criterion) {
    c.bench_function("insert_string", |b| b.iter(|| test_insert_string()));
}

criterion_group!(benches, benchmark_insert_str, benchmark_insert_string);
criterion_main!(benches);
