//! The rules the shared assets keep: present, one theme mechanism, every
//! colour a token, every token an app reads declared.

use castle_design::{CASTLE_WEBP, CSS, VERSION};

/// Tokens the stylesheet reads but leaves to the app or the markup (the
/// header of castle.css lists them): app accents, the per-element hue and
/// lamp fill.
const APP_TOKENS: &[&str] = &[
    "--on",
    "--kelvin",
    "--ink-dark",
    "--ink-light",
    "--brand",
    "--brand-text",
    "--brand-soft",
    "--avatar-bg",
    "--feel",
    "--hue",
    "--fill",
];

/// The stylesheet without its comments.
fn css_code() -> String {
    strip_comments(CSS)
}

/// CSS without its comments.
fn strip_comments(css: &str) -> String {
    let mut out = String::with_capacity(css.len());
    let mut rest = css;
    while let Some(start) = rest.find("/*") {
        out.push_str(&rest[..start]);
        let end = rest[start..].find("*/").expect("unterminated comment");
        rest = &rest[start + end + 2..];
    }
    out.push_str(rest);
    out
}

/// Every declaration (`name: value`) in the sheet, at any nesting depth.
fn declarations(code: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut segment = String::new();
    for c in code.chars() {
        match c {
            // what came before an opening brace is a selector or an at-rule
            '{' => segment.clear(),
            ';' | '}' => {
                if let Some((name, value)) = segment.split_once(':') {
                    out.push((name.trim().to_owned(), value.trim().to_owned()));
                }
                segment.clear();
            }
            c => segment.push(c),
        }
    }
    out
}

/// The colour literal in a value, if any: a hex colour, a colour function or
/// a named colour.
fn colour_literal(value: &str) -> Option<String> {
    let lower = value.to_ascii_lowercase();
    let bytes = lower.as_bytes();
    for (i, _) in lower.match_indices('#') {
        let hex = bytes[i + 1..]
            .iter()
            .take_while(|b| b.is_ascii_hexdigit())
            .count();
        if matches!(hex, 3 | 4 | 6 | 8) {
            return Some(lower[i..i + 1 + hex].to_owned());
        }
    }
    let ident = |b: u8| b.is_ascii_alphanumeric() || b == b'-' || b == b'_';
    for word in lower.split(|c: char| !ident(c as u8) && c != '(') {
        const FUNCTIONS: &[&str] = &[
            "rgb(", "rgba(", "hsl(", "hsla(", "hwb(", "lab(", "lch(", "oklab(", "oklch(", "color(",
        ];
        // every CSS named colour (CSS Color 4); transparent and currentColor
        // are not literals and stay allowed
        const NAMED: &[&str] = &[
            "aliceblue",
            "antiquewhite",
            "aqua",
            "aquamarine",
            "azure",
            "beige",
            "bisque",
            "black",
            "blanchedalmond",
            "blue",
            "blueviolet",
            "brown",
            "burlywood",
            "cadetblue",
            "chartreuse",
            "chocolate",
            "coral",
            "cornflowerblue",
            "cornsilk",
            "crimson",
            "cyan",
            "darkblue",
            "darkcyan",
            "darkgoldenrod",
            "darkgray",
            "darkgreen",
            "darkgrey",
            "darkkhaki",
            "darkmagenta",
            "darkolivegreen",
            "darkorange",
            "darkorchid",
            "darkred",
            "darksalmon",
            "darkseagreen",
            "darkslateblue",
            "darkslategray",
            "darkslategrey",
            "darkturquoise",
            "darkviolet",
            "deeppink",
            "deepskyblue",
            "dimgray",
            "dimgrey",
            "dodgerblue",
            "firebrick",
            "floralwhite",
            "forestgreen",
            "fuchsia",
            "gainsboro",
            "ghostwhite",
            "gold",
            "goldenrod",
            "gray",
            "green",
            "greenyellow",
            "grey",
            "honeydew",
            "hotpink",
            "indianred",
            "indigo",
            "ivory",
            "khaki",
            "lavender",
            "lavenderblush",
            "lawngreen",
            "lemonchiffon",
            "lightblue",
            "lightcoral",
            "lightcyan",
            "lightgoldenrodyellow",
            "lightgray",
            "lightgreen",
            "lightgrey",
            "lightpink",
            "lightsalmon",
            "lightseagreen",
            "lightskyblue",
            "lightslategray",
            "lightslategrey",
            "lightsteelblue",
            "lightyellow",
            "lime",
            "limegreen",
            "linen",
            "magenta",
            "maroon",
            "mediumaquamarine",
            "mediumblue",
            "mediumorchid",
            "mediumpurple",
            "mediumseagreen",
            "mediumslateblue",
            "mediumspringgreen",
            "mediumturquoise",
            "mediumvioletred",
            "midnightblue",
            "mintcream",
            "mistyrose",
            "moccasin",
            "navajowhite",
            "navy",
            "oldlace",
            "olive",
            "olivedrab",
            "orange",
            "orangered",
            "orchid",
            "palegoldenrod",
            "palegreen",
            "paleturquoise",
            "palevioletred",
            "papayawhip",
            "peachpuff",
            "peru",
            "pink",
            "plum",
            "powderblue",
            "purple",
            "rebeccapurple",
            "red",
            "rosybrown",
            "royalblue",
            "saddlebrown",
            "salmon",
            "sandybrown",
            "seagreen",
            "seashell",
            "sienna",
            "silver",
            "skyblue",
            "slateblue",
            "slategray",
            "slategrey",
            "snow",
            "springgreen",
            "steelblue",
            "tan",
            "teal",
            "thistle",
            "tomato",
            "turquoise",
            "violet",
            "wheat",
            "white",
            "whitesmoke",
            "yellow",
            "yellowgreen",
        ];
        if let Some(f) = FUNCTIONS.iter().find(|f| word.starts_with(**f)) {
            return Some((*f).to_owned());
        }
        if NAMED.contains(&word) {
            return Some(word.to_owned());
        }
    }
    None
}

#[test]
fn constants_are_present() {
    assert!(CSS.len() > 1000, "castle.css looks empty");
    assert!(!VERSION.is_empty());
    assert!(CASTLE_WEBP.len() > 1000, "castle.webp looks empty");
    assert_eq!(&CASTLE_WEBP[..4], b"RIFF");
    assert_eq!(&CASTLE_WEBP[8..12], b"WEBP");
}

#[test]
fn braces_balance() {
    let code = css_code();
    assert_eq!(code.matches('{').count(), code.matches('}').count());
}

#[test]
fn themes_come_from_light_dark_only() {
    let code = css_code();
    assert!(
        !code.contains("prefers-color-scheme"),
        "the theme is chosen by color-scheme and light-dark(), never a media query"
    );
    assert!(code.contains("[data-theme=\"light\"] { color-scheme: light; }"));
    assert!(code.contains("[data-theme=\"dark\"] { color-scheme: dark; }"));
}

#[test]
fn every_colour_is_a_token() {
    let offenders: Vec<String> = declarations(&css_code())
        .into_iter()
        .filter(|(name, _)| !name.starts_with("--"))
        .filter_map(|(name, value)| {
            colour_literal(&value).map(|lit| format!("{name}: {value}  ({lit})"))
        })
        .collect();
    assert!(
        offenders.is_empty(),
        "colour literals outside a token:\n{}",
        offenders.join("\n")
    );
}

#[test]
fn the_literal_check_catches_colours() {
    assert_eq!(colour_literal("1px solid #fff").as_deref(), Some("#fff"));
    assert_eq!(colour_literal("rgb(0 0 0 / .5)").as_deref(), Some("rgb("));
    assert_eq!(colour_literal("white").as_deref(), Some("white"));
    assert_eq!(colour_literal("navy").as_deref(), Some("navy"));
    assert_eq!(
        colour_literal("1px solid RebeccaPurple").as_deref(),
        Some("rebeccapurple")
    );
    assert_eq!(colour_literal("transparent"), None);
    assert_eq!(colour_literal("currentColor"), None);
    assert_eq!(colour_literal("var(--accent)"), None);
    assert_eq!(colour_literal("var(--err-text)"), None);
    assert_eq!(
        colour_literal("color-mix(in srgb, var(--ok) 15%, transparent)"),
        None
    );
    assert_eq!(colour_literal("attr(data-label) \" \""), None);
}

#[test]
fn every_token_read_is_declared() {
    let code = css_code();
    let declared: Vec<String> = declarations(&code)
        .into_iter()
        .map(|(name, _)| name)
        .filter(|name| name.starts_with("--"))
        .collect();
    let mut missing = Vec::new();
    for (i, _) in code.match_indices("var(") {
        let name: String = code[i + 4..]
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect();
        if !declared.contains(&name) && !APP_TOKENS.contains(&name.as_str()) {
            missing.push(name);
        }
    }
    missing.dedup();
    assert!(
        missing.is_empty(),
        "tokens read but never declared: {missing:?}"
    );
}

#[test]
fn no_mockup_scaffolding_left() {
    let code = css_code();
    for leftover in ["@container", ".screen", ".frame", "cqw", "cqh", ".app-"] {
        assert!(
            !code.contains(leftover),
            "mockup-only {leftover} in castle.css"
        );
    }
}

/// Every top-level rule in source order: the `@media` query it sits in (if
/// any) and each of its selectors.
fn rules(code: &str) -> Vec<(Option<String>, Vec<String>)> {
    let mut out = Vec::new();
    let mut media: Option<String> = None;
    let mut depth = 0;
    let mut segment = String::new();
    for c in code.chars() {
        match c {
            '{' => {
                let head = segment.trim().to_owned();
                if depth == 0 && head.starts_with('@') {
                    media = Some(head);
                } else if depth == usize::from(media.is_some()) {
                    let selectors = head.split(',').map(|s| s.trim().to_owned()).collect();
                    out.push((media.clone(), selectors));
                }
                depth += 1;
                segment.clear();
            }
            '}' => {
                depth -= 1;
                if depth == 0 {
                    media = None;
                }
                segment.clear();
            }
            ';' => segment.clear(),
            c => segment.push(c),
        }
    }
    out
}

#[test]
fn media_overrides_follow_their_base_rule() {
    assert_overrides_follow_base("castle.css", &css_code());
}

#[test]
fn mockup_overrides_follow_their_base_rule() {
    let mockup = include_str!("../docs/mockup.html");
    let start = mockup
        .find(r#"<style id="castle-css">"#)
        .expect("mockup sheet")
        + 23;
    let end = start
        + mockup[start..]
            .find("</style>")
            .expect("unterminated style");
    assert_overrides_follow_base("the mockup", &strip_comments(&mockup[start..end]));
}

/// Fails when an at-rule overrides a selector whose base rule comes later.
fn assert_overrides_follow_base(sheet: &str, code: &str) {
    let rules = rules(code);
    let mut misplaced = Vec::new();
    for (i, (media, selectors)) in rules.iter().enumerate() {
        let Some(media) = media else { continue };
        for selector in selectors {
            let later_base = rules[i + 1..]
                .iter()
                .any(|(m, s)| m.is_none() && s.contains(selector));
            if later_base {
                misplaced.push(format!("{selector} in {media}"));
            }
        }
    }
    assert!(
        misplaced.is_empty(),
        "{sheet}: overrides that come before the base rule they override: {misplaced:?}"
    );
}
