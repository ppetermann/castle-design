//! The ircord theme file restates castle.css's palette as seeds. These tests
//! hold it to that: every seed ircord's importer needs is present in both
//! modes, and each seed that is a castle token equals castle.css's value for
//! that mode, so a colour change in the sheet fails here until the theme
//! follows.

const THEME: &str = include_str!("../assets/ircord-castle.ircord-theme.json");
const CSS: &str = castle_design::CSS;

/// (seed in the theme, castle.css token), as a path into the mode's object.
const MAPPED: [(&str, &str); 10] = [
    ("canvas", "bg-sunken"),
    ("surface", "surface"),
    ("ink", "text"),
    ("muted", "muted"),
    ("accent", "accent"),
    ("red", "err"),
    ("orange", "warn"),
    ("green", "ok"),
    ("blue", "info"),
    ("purple", "ask"),
];
/// Seeds that are not castle tokens, still required by the importer.
const CHOSEN: [&str; 4] = ["overlay", "cyan", "yellow", "pink"];

/// The text of one mode's colour object.
fn mode(name: &str) -> &'static str {
    let start = THEME
        .find(&format!("\"{name}\": {{"))
        .unwrap_or_else(|| panic!("no {name} colours"));
    let rest = &THEME[start..];
    // The palette object closes the mode: two closing braces end it.
    let end = rest.find("}\n    }").expect("the mode's object ends");
    &rest[..end]
}

fn seed(mode: &str, key: &str) -> String {
    let at = mode
        .find(&format!("\"{key}\": \"#"))
        .unwrap_or_else(|| panic!("theme lacks {key}"));
    mode[at..].split('"').nth(3).unwrap().to_string()
}

/// castle.css's value of `--token` for light (0) or dark (1).
fn token(name: &str, dark: bool) -> String {
    let at = CSS
        .find(&format!("--{name}:"))
        .unwrap_or_else(|| panic!("castle.css lost --{name}"));
    let pair = &CSS[at..][CSS[at..].find("light-dark(").unwrap() + 11..];
    let pair = &pair[..pair.find(')').unwrap()];
    pair.split(',')
        .nth(usize::from(dark))
        .unwrap()
        .trim()
        .to_string()
}

#[test]
fn theme_seeds_are_castle_tokens() {
    for (name, dark) in [("light", false), ("dark", true)] {
        let m = mode(name);
        for (key, tok) in MAPPED {
            assert_eq!(seed(m, key), token(tok, dark), "{name} {key} vs --{tok}");
        }
    }
}

#[test]
fn theme_file_has_every_seed_ircord_needs_in_both_modes() {
    assert!(THEME.contains("\"ircordTheme\": 1"));
    for name in ["light", "dark"] {
        let m = mode(name);
        for key in MAPPED.iter().map(|(k, _)| *k).chain(CHOSEN) {
            let v = seed(m, key);
            assert!(
                v.len() == 7 && v.starts_with('#') && v[1..].chars().all(|c| c.is_ascii_hexdigit()),
                "{name} {key} is not a #rrggbb colour: {v}"
            );
        }
    }
    assert!(
        !THEME.contains("url("),
        "ircord strips url() from theme CSS"
    );
}

#[test]
fn theme_css_root_rule_wins_over_ircord_appearance() {
    // ircord writes its settings as `:root:root`; a plain `:root` loses.
    assert!(THEME.contains(":root:root {"));
    assert!(!THEME.contains("\\n:root {"));
}
