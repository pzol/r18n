extern crate r18n;

pub fn main() {
    println!("r18n");
    match r18n::load("tests/pl-pl.toml") {
        Ok(r) => println!("all good\n{:?}", r.t("tlc.FRA.name")),
        Err(e) => println!("error {:?}", e)
    }
}
