extern crate r18n;

fn load_example() -> r18n::R18n {
    r18n::load("tests/en.toml").unwrap()
}

#[test]
fn lookup() {
    let r = load_example();
    assert_eq!(r.t("tlc.FRA.name").unwrap(), "Frankfurt");
}

#[test]
fn errors() {
    use r18n::LookupError as E;
    let r = load_example();

    assert_eq!(r.t("i_dont_exist").unwrap_err(), E::NotFound);
    assert_eq!(r.t("pax.guests").unwrap_err(), E::NotAString);
}

#[test]
fn count() {
    let r = load_example();
    // assert_eq!(r.c("pax.guests", 0).unwrap(), "0 guests");
    assert_eq!(r.c("pax.guests", 1).unwrap(), "1 guest");
    assert_eq!(r.c("pax.guests", 2).unwrap(), "2 guests");
    assert_eq!(r.c("pax.guests", 5).unwrap(), "5 guests");
}

#[test]
fn template() {
    let r = load_example();
}
