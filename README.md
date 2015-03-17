# r18n

## Text `t`

```rust
extern crate r18n;
let toml = r#"
[tlc.FRA]
name = "Frankfurt"
"#;

let r : r18n::R18n = toml.parse().unwrap();
assert_eq!(r.t("tlc.FRA.name").unwrap(), "Frankfurt");

```

## Count

```rust
extern crate r18n;
let toml = r#"
[pax]
guests = [
    "1 guest",
    "%d guests"
]
"#;

let r : r18n::R18n = toml.parse().unwrap();
assert_eq!(r.c("pax.guests", 1).unwrap(), "1 guest");
assert_eq!(r.c("pax.guests", 3).unwrap(), "3 guests");

```


## Links

* https://www.gnu.org/software/gettext/manual/gettext.html#C-Sources-Context
* http://docs.translatehouse.org/projects/localization-guide/en/latest/l10n/pluralforms.html?id=l10n/pluralforms
* http://unicode.org/repos/cldr-tmp/trunk/diff/supplemental/language_plural_rules.html
* https://github.com/ai/r18n

