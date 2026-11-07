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

/// Every rule's selector list and body, innermost head only (a media
/// query's own head sits before the rule's and is cut off at its brace).
fn bodies(code: &str) -> Vec<(String, String)> {
    code.split('}')
        .filter_map(|rule| {
            let (head, body) = rule.rsplit_once('{')?;
            let head = head.rsplit_once('{').map_or(head, |(_, h)| h);
            Some((head.trim().to_owned(), body.trim().to_owned()))
        })
        .collect()
}

/// The number a `--z-*` token is set to.
fn z(code: &str, name: &str) -> i32 {
    declarations(code)
        .into_iter()
        .find(|(n, _)| n == name)
        .unwrap_or_else(|| panic!("{name} is not declared"))
        .1
        .parse()
        .unwrap_or_else(|_| panic!("{name} is not a number"))
}

#[test]
fn layers_stack_in_order() {
    // a confirm opened from a sheet: its scrim (.scrim.above) must cover the
    // sheet, and the confirm its scrim; the docked sheet stays under the bars
    for (sheet, code) in [("castle.css", css_code()), ("the mockup", mockup_css())] {
        let order = [
            "--z-page",
            "--z-docked",
            "--z-bar",
            "--z-menu",
            "--z-scrim",
            "--z-sheet",
            "--z-toast",
            "--z-dialog-scrim",
            "--z-dialog",
        ];
        for pair in order.windows(2) {
            assert!(
                z(&code, pair[0]) < z(&code, pair[1]),
                "{sheet}: {} is not under {}",
                pair[0],
                pair[1]
            );
        }
        for (selector, token) in [
            (".scrim", "--z-scrim"),
            (".scrim.above", "--z-dialog-scrim"),
            (".dialog:not(dialog)", "--z-dialog"),
            (".sheet.docked", "--z-docked"),
        ] {
            let body = bodies(&code)
                .into_iter()
                .find(|(head, _)| head == selector)
                .unwrap_or_else(|| panic!("{sheet}: no rule {selector}"))
                .1;
            assert!(
                body.contains(&format!("z-index: var({token})")),
                "{sheet}: {selector} is not on {token}"
            );
        }
    }
}

#[test]
fn a_sidebar_page_grows_on_an_ultra_wide_screen() {
    // .with-sidenav main.page outranks main.page, so the ultra-wide rule must
    // name it too or a sidebar app stays at its desk width
    for (sheet, code, wide) in [
        ("castle.css", css_code(), "@media (width >= 100rem)"),
        (
            "the mockup",
            mockup_css(),
            "@container frame (min-width: 100rem)",
        ),
    ] {
        let grows = rules(&code).into_iter().any(|(media, selectors)| {
            media.as_deref() == Some(wide)
                && selectors.iter().any(|s| s == ".with-sidenav main.page")
        });
        assert!(
            grows,
            "{sheet}: no ultra-wide rule for .with-sidenav main.page"
        );
    }
}

#[test]
fn a_docked_sheet_makes_room_beside_the_page() {
    for (sheet, code) in [("castle.css", css_code()), ("the mockup", mockup_css())] {
        let room = bodies(&code).into_iter().any(|(head, body)| {
            head.contains(":has(.sheet.docked")
                && head.ends_with("main.page")
                && body.contains("padding-right: calc(var(--sheet-w)")
        });
        assert!(room, "{sheet}: nothing pads the page for a docked sheet");
    }
}

#[test]
fn every_status_piece_has_every_tone() {
    // a dot, a badge, a note and a toast each in ok, warn, err, info and ask,
    // and a count in err (the default) or ask
    for (sheet, code) in [("castle.css", css_code()), ("the mockup", mockup_css())] {
        let heads: Vec<String> = rules(&code)
            .into_iter()
            .flat_map(|(_, selectors)| selectors)
            .collect();
        for piece in [".dot", ".badge", ".note", ".toast"] {
            for tone in ["ok", "warn", "err", "info", "ask"] {
                let selector = format!("{piece}.{tone}");
                assert!(heads.contains(&selector), "{sheet}: no {selector}");
            }
        }
        for selector in ["nav.tabs .count.ask", ".nav-badge.ask"] {
            assert!(
                heads.iter().any(|h| h == selector),
                "{sheet}: no {selector}"
            );
        }
    }
}

#[test]
fn small_uppercase_labels_share_one_tracking() {
    for (sheet, code) in [("castle.css", css_code()), ("the mockup", mockup_css())] {
        let off: Vec<String> = bodies(&code)
            .into_iter()
            .filter(|(_, body)| body.contains("text-transform: uppercase"))
            .filter(|(_, body)| !body.contains("letter-spacing: var(--track-caps)"))
            .map(|(head, _)| head)
            .collect();
        assert!(
            off.is_empty(),
            "{sheet}: uppercase labels off --track-caps: {off:?}"
        );
    }
}

#[test]
fn tiles_and_lamps_are_on_the_type_scale() {
    // density reaches a component only through the --t-* tokens
    for (sheet, code) in [("castle.css", css_code()), ("the mockup", mockup_css())] {
        let off: Vec<String> = bodies(&code)
            .into_iter()
            .filter(|(head, _)| {
                [".tile", ".metric", ".lamp"]
                    .iter()
                    .any(|p| head.starts_with(p))
            })
            .filter(|(head, body)| {
                head != ".lamp-tile:has(.lamp-dim) .lamp"
                    && body
                        .split(';')
                        .any(|d| d.trim().starts_with("font-size:") && !d.contains("var(--t-"))
            })
            .map(|(head, _)| head)
            .collect();
        assert!(off.is_empty(), "{sheet}: font sizes off the scale: {off:?}");
    }
}

#[test]
fn inputs_do_not_make_ios_zoom() {
    // iOS zooms the page on focus into a field whose text is under 16px
    assert!(
        css_code()
            .contains("@media (pointer: coarse) { .input { font-size: max(1rem, var(--t-m)); } }")
    );
}

#[test]
fn form_states_are_drawn() {
    let heads: Vec<String> = rules(&css_code())
        .into_iter()
        .flat_map(|(_, selectors)| selectors)
        .collect();
    for selector in [
        ".input:disabled",
        ".input[readonly]",
        r#".input[aria-invalid="true"]"#,
        ".field .error",
        ".field .req",
        "select.input",
        "progress.progress",
        ".spinner",
        ".htmx-indicator",
    ] {
        assert!(heads.iter().any(|h| h == selector), "no {selector}");
    }
}

/// A mockup selector list, with the frame scoping taken off, matches the
/// list of a castle.css rule that holds any of its selectors: a selector the
/// mockup adds to a shared rule (say `.screen select.input` beside
/// `.input`) raises that rule's specificity above its own variants
/// (`.input:focus`) and draws the mockup differently from the sheet.
#[test]
fn the_mockups_rules_list_the_sheets_selectors() {
    fn unscoped(selector: &str) -> String {
        let s = selector.split_whitespace().collect::<Vec<_>>().join(" ");
        for scope in [":where(.screen) ", ".screen "] {
            if let Some(rest) = s.strip_prefix(scope) {
                return rest.to_owned();
            }
        }
        s
    }
    fn lists(code: &str) -> Vec<Vec<String>> {
        rules(code)
            .into_iter()
            .map(|(_, selectors)| {
                let mut list: Vec<String> = selectors
                    .iter()
                    .map(|s| unscoped(s))
                    .filter(|s| s != ".frame" && !s.is_empty())
                    .collect();
                list.sort();
                list.dedup();
                list
            })
            .collect()
    }
    let sheet = lists(&css_code());
    let mut drift = Vec::new();
    for list in lists(&mockup_css()) {
        // the mockup writes some base element rules as :where(.screen) :is(…)
        if list
            .iter()
            .any(|s| s.starts_with(":is(") || s.starts_with(":where("))
        {
            continue;
        }
        let partners: Vec<&Vec<String>> = sheet
            .iter()
            .filter(|l| list.iter().any(|s| l.contains(s)))
            .collect();
        if !partners.is_empty() && !partners.contains(&&list) {
            drift.push(format!("{list:?} (castle.css: {:?})", partners[0]));
        }
    }
    assert!(
        drift.is_empty(),
        "mockup rules that differ from castle.css's:\n{}",
        drift.join("\n")
    );
}

#[test]
fn the_shared_icons_are_the_mockups() {
    // each icon is a whole inline svg.icon, hidden from assistive technology,
    // drawing the same shapes as the mockup's symbol, so the two never drift
    let mockup = include_str!("../docs/mockup.html");
    for (icon, symbol) in [
        (castle_design::icons::HOME, "i-home"),
        (castle_design::icons::MONITOR, "i-monitor"),
        (castle_design::icons::SUN, "i-sun"),
        (castle_design::icons::MOON, "i-moon"),
    ] {
        let open = r#"<svg class="icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" aria-hidden="true" focusable="false">"#;
        let shapes = icon
            .strip_prefix(open)
            .and_then(|rest| rest.strip_suffix("</svg>"))
            .unwrap_or_else(|| panic!("{symbol}: not a bare svg.icon: {icon}"));
        let head = format!(r#"<symbol id="{symbol}" viewBox="0 0 24 24">"#);
        let start = mockup.find(&head).expect("the mockup's symbol") + head.len();
        let end = start + mockup[start..].find("</symbol>").unwrap();
        assert_eq!(
            shapes,
            &mockup[start..end],
            "{symbol} differs from the mockup"
        );
    }
}

#[test]
fn every_wash_is_a_named_tint() {
    // a colour mixed over its ground reads a --tint-* strength; the only
    // literal percentages are the design language's three non-tint sites, each
    // pinned to its rule: a lit lamp's glow, a danger card's rim and the quiet
    // backdrop's wash
    const SITES: [(&str, &str); 4] = [
        (".lamp.on", "var(--fill) 60%"),
        (".lamp.on", "var(--fill) 35%"),
        (".card.danger", "var(--err) 50%"),
        (".backdrop.quiet::after", "var(--bg) 55%"),
    ];
    let code = css_code();
    let mut seen = Vec::new();
    let mut literal = Vec::new();
    for (i, _) in code.match_indices("color-mix(in srgb, var(") {
        let rest = &code[i..];
        let mix = rest["color-mix(in srgb, ".len()..]
            .split(',')
            .next()
            .unwrap();
        if !mix.ends_with('%') {
            continue;
        }
        let head = &code[..i];
        let open = head.rfind('{').unwrap();
        let start = head[..open].rfind(['}', '\n']).map_or(0, |s| s + 1);
        let selector = head[start..open].trim();
        if SITES.contains(&(selector, mix)) {
            seen.push((selector, mix));
        } else {
            literal.push(format!("{selector}: {}", &rest[..rest.find(';').unwrap()]));
        }
    }
    assert!(
        literal.is_empty(),
        "washes off the tint scale (a non-tint site is listed by rule):\n{}",
        literal.join("\n")
    );
    for site in SITES {
        assert!(
            seen.contains(&site),
            "the non-tint wash {site:?} is gone or retuned: update the design language's tint table and this list"
        );
    }
}
