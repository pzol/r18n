#![feature(test)]
extern crate r18n;
extern crate test;

#[bench]
fn bench_lookup(b: &mut test::Bencher) {
    let r = r18n::load("tests/en.toml").unwrap();

    b.iter(|| {
        r.t("tlc.FRA.name")
    })
}

#[bench]
fn bench_count(b: &mut test::Bencher) {
    let r = r18n::load("tests/en.toml").unwrap();

    b.iter(|| {
        r.c("pax.guests", 2)
    })
}
