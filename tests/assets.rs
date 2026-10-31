//! The rules the shared assets keep: present, one theme mechanism, every
//! colour a token, every token an app reads declared.

use castle_design::{CASTLE_WEBP, CSS, VERSION};

/// Tokens the stylesheet reads but leaves to the app or the markup (the
/// header of castle.css lists them): app accents, the per-element hue and
/// lamp fill, a time-grid slot's place and the now line's hour.
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
    "--start",
    "--len",
    "--lane",
    "--lanes",
    "--at",
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

/// The mockup's frame-scoped copy of the sheet, without its comments.
fn mockup_css() -> String {
    let mockup = include_str!("../docs/mockup.html");
    let start = mockup
        .find(r#"<style id="castle-css">"#)
        .expect("mockup sheet")
        + 23;
    let end = start
        + mockup[start..]
            .find("</style>")
            .expect("unterminated style");
    strip_comments(&mockup[start..end])
}

#[test]
fn mockup_overrides_follow_their_base_rule() {
    assert_overrides_follow_base("the mockup", &mockup_css());
}

/// Selectors that take any `aria-current` as the current one, so a link a
/// template marks `aria-current="false"` would be styled as chosen.
fn bare_aria_current(code: &str) -> Vec<String> {
    rules(code)
        .into_iter()
        .flat_map(|(_, selectors)| selectors)
        .filter(|s| {
            s.match_indices("[aria-current]")
                .any(|(i, m)| !s[i + m.len()..].starts_with(r#":not([aria-current="false"])"#))
        })
        .collect()
}

#[test]
fn aria_current_false_is_not_current() {
    for (sheet, code) in [("castle.css", css_code()), ("the mockup", mockup_css())] {
        let bare = bare_aria_current(&code);
        assert!(bare.is_empty(), "bare [aria-current] in {sheet}: {bare:?}");
    }
    assert_eq!(
        bare_aria_current(".pages > [aria-current] { color: red; }"),
        [".pages > [aria-current]"]
    );
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

/// The states a `.state-card` colours: each class that sets `--tone-text`
/// must also be in the `:is()` list that colours its badge, and the other way
/// round, or a state would get a coloured bar beside a plain badge.
#[test]
fn state_card_states_colour_their_badge() {
    for (sheet, code) in [("castle.css", css_code()), ("the mockup", mockup_css())] {
        // each rule is "head { body" between closing braces (a media query's
        // own head sits before the rule's and is cut off at its brace)
        let mut toned: Vec<String> = Vec::new();
        for rule in code.split('}') {
            let Some((head, body)) = rule.rsplit_once('{') else {
                continue;
            };
            let head = head.rsplit_once('{').map_or(head, |(_, h)| h);
            if !body.contains("--tone-text:") {
                continue;
            }
            for selector in head.split(',') {
                let class = selector.trim().strip_prefix(".state-card.");
                toned.push(class.expect("a state class").to_owned());
            }
        }
        let start = code.find(".state-card:is(").expect("the badge rule") + 15;
        let end = start + code[start..].find(')').unwrap();
        let mut listed: Vec<String> = code[start..end]
            .split(',')
            .map(|s| s.trim().trim_start_matches('.').to_owned())
            .collect();
        toned.sort();
        listed.sort();
        assert_eq!(
            toned, listed,
            "{sheet}: state card tones and badge list differ"
        );
        assert_eq!(toned.len(), 7, "{sheet}: {toned:?}");
    }
}
