//! WCAG contrast of the sheet's colour tokens, computed from castle.css
//! itself: every text token on every surface it sits on, and the hue tokens
//! over the whole hue range, in both themes. AA is 4.5:1 for words and 3:1
//! for a line or dot that carries meaning and for large text.

use castle_design::CSS;

const AA_TEXT: f64 = 4.5;
const AA_LARGE_OR_LINE: f64 = 3.0;

#[derive(Clone, Copy, PartialEq)]
enum Theme {
    Light,
    Dark,
}

/// A colour with straight (not premultiplied) alpha, channels 0..=1.
#[derive(Clone, Copy, Debug)]
struct Rgba(f64, f64, f64, f64);

impl Rgba {
    /// This colour laid over an opaque one.
    fn over(self, below: Rgba) -> Rgba {
        let a = self.3;
        Rgba(
            self.0 * a + below.0 * (1.0 - a),
            self.1 * a + below.1 * (1.0 - a),
            self.2 * a + below.2 * (1.0 - a),
            1.0,
        )
    }

    fn luminance(self) -> f64 {
        let lin = |c: f64| {
            if c <= 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * lin(self.0) + 0.7152 * lin(self.1) + 0.0722 * lin(self.2)
    }
}

fn contrast(a: Rgba, b: Rgba) -> f64 {
    let (la, lb) = (a.luminance(), b.luminance());
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

/// The first declaration of a custom property in the sheet.
fn token(name: &str) -> String {
    let needle = format!("{name}:");
    let mut rest = CSS;
    loop {
        let i = rest
            .find(&needle)
            .unwrap_or_else(|| panic!("{name} is not declared"));
        let before = rest[..i].chars().last();
        // a whole name: not the tail of a longer one such as --x-name
        if matches!(before, Some(c) if c.is_whitespace() || c == '{' || c == ';') {
            let value = &rest[i + needle.len()..];
            let end = value.find(';').expect("unterminated declaration");
            return value[..end].trim().to_owned();
        }
        rest = &rest[i + needle.len()..];
    }
}

/// `inner` split at its top-level commas.
fn args(inner: &str) -> Vec<&str> {
    let (mut out, mut depth, mut start) = (Vec::new(), 0, 0);
    for (i, c) in inner.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => {
                out.push(inner[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    out.push(inner[start..].trim());
    out
}

/// The text between `name(` and its matching `)`, when `value` is that call.
fn call<'a>(value: &'a str, name: &str) -> Option<&'a str> {
    let inner = value.strip_prefix(name)?.strip_prefix('(')?;
    inner.strip_suffix(')')
}

fn hsl(h: f64, s: f64, l: f64) -> Rgba {
    let (s, l) = (s / 100.0, l / 100.0);
    let k = |n: f64| (n + h / 30.0).rem_euclid(12.0);
    let a = s * l.min(1.0 - l);
    let f = |n: f64| l - a * (k(n) - 3.0).min(9.0 - k(n)).clamp(-1.0, 1.0);
    Rgba(f(0.0), f(8.0), f(4.0), 1.0)
}

/// Evaluates the colour expressions castle.css uses: hex, rgb(), hsl() of
/// --hue, light-dark(), color-mix(in srgb, …), var() and transparent.
fn eval(value: &str, theme: Theme, hue: f64) -> Rgba {
    let value = value.trim();
    if let Some(name) = call(value, "var") {
        let name = args(name)[0];
        return eval(&token(name), theme, hue);
    }
    if let Some(inner) = call(value, "light-dark") {
        let a = args(inner);
        return eval(if theme == Theme::Light { a[0] } else { a[1] }, theme, hue);
    }
    if let Some(inner) = call(value, "color-mix") {
        let a = args(inner);
        assert_eq!(a[0], "in srgb", "only srgb mixes: {value}");
        let (first, p) = a[1].rsplit_once(' ').expect("a mix percentage");
        let p: f64 = p.trim_end_matches('%').parse::<f64>().unwrap() / 100.0;
        let (x, y) = (eval(first, theme, hue), eval(a[2], theme, hue));
        // CSS mixes premultiplied colours, then un-premultiplies
        let alpha = x.3 * p + y.3 * (1.0 - p);
        let ch = |cx: f64, cy: f64| {
            if alpha == 0.0 {
                0.0
            } else {
                (cx * x.3 * p + cy * y.3 * (1.0 - p)) / alpha
            }
        };
        return Rgba(ch(x.0, y.0), ch(x.1, y.1), ch(x.2, y.2), alpha);
    }
    if let Some(inner) = call(value, "hsl") {
        let parts: Vec<&str> = inner.split(')').collect();
        assert!(
            inner.starts_with("var(--hue"),
            "hsl() of --hue only: {value}"
        );
        let sl: Vec<f64> = parts[1]
            .split_whitespace()
            .map(|p| p.trim_end_matches('%').parse().unwrap())
            .collect();
        return hsl(hue, sl[0], sl[1]);
    }
    if let Some(inner) = call(value, "rgb") {
        let (rgb, a) = inner.split_once('/').unwrap_or((inner, "1"));
        let c: Vec<f64> = rgb
            .split_whitespace()
            .map(|n| n.parse::<f64>().unwrap() / 255.0)
            .collect();
        return Rgba(c[0], c[1], c[2], a.trim().parse().unwrap());
    }
    if value == "transparent" {
        return Rgba(0.0, 0.0, 0.0, 0.0);
    }
    let hex = value
        .strip_prefix('#')
        .unwrap_or_else(|| panic!("not a colour: {value}"));
    assert_eq!(hex.len(), 6, "six-digit hex only: {value}");
    let ch = |i: usize| f64::from(u8::from_str_radix(&hex[i..i + 2], 16).unwrap()) / 255.0;
    Rgba(ch(0), ch(2), ch(4), 1.0)
}

/// The grounds words sit on: the page, an opaque surface, the second
/// surface and strong glass over the page (the castle under the glass is
/// washed toward --bg, so --bg stands in for it).
fn grounds(theme: Theme) -> Vec<(&'static str, Rgba)> {
    let bg = eval("var(--bg)", theme, 0.0);
    vec![
        ("--bg", bg),
        ("--surface", eval("var(--surface)", theme, 0.0)),
        ("--surface-2", eval("var(--surface-2)", theme, 0.0)),
        (
            "--glass-strong",
            eval("var(--glass-strong)", theme, 0.0).over(bg),
        ),
    ]
}

struct Check(Vec<String>);

impl Check {
    fn pair(&mut self, what: &str, theme: Theme, fg: Rgba, bg: Rgba, floor: f64) {
        let ratio = contrast(fg.over(bg), bg);
        if ratio < floor {
            let theme = if theme == Theme::Light {
                "light"
            } else {
                "dark"
            };
            self.0
                .push(format!("{what} ({theme}): {ratio:.2} < {floor}"));
        }
    }

    fn done(self) {
        assert!(self.0.is_empty(), "below WCAG AA:\n{}", self.0.join("\n"));
    }
}

const THEMES: [Theme; 2] = [Theme::Light, Theme::Dark];

#[test]
fn text_tokens_meet_aa_on_every_surface() {
    let mut check = Check(Vec::new());
    for theme in THEMES {
        for text in [
            "--text",
            "--text-soft",
            "--muted",
            "--accent-text",
            "--ok-text",
            "--warn-text",
            "--err-text",
            "--info",
        ] {
            let fg = eval(&format!("var({text})"), theme, 0.0);
            for (ground, bg) in grounds(theme) {
                check.pair(&format!("{text} on {ground}"), theme, fg, bg, AA_TEXT);
            }
        }
        // a badge: its words on its own wash (castle.css's .badge rules), over a surface or glass
        for (text, wash, pct) in [
            ("--ok-text", "--ok", 15),
            ("--warn-text", "--warn", 15),
            ("--err-text", "--err", 15),
            ("--info", "--info", 15),
            ("--muted", "--muted", 12),
        ] {
            let fg = eval(&format!("var({text})"), theme, 0.0);
            let mix = format!("color-mix(in srgb, var({wash}) {pct}%, transparent)");
            for (ground, bg) in grounds(theme) {
                let bg = eval(&mix, theme, 0.0).over(bg);
                check.pair(
                    &format!("{text} on a {wash} badge over {ground}"),
                    theme,
                    fg,
                    bg,
                    AA_TEXT,
                );
            }
        }
        let accent_soft = eval("var(--accent-soft)", theme, 0.0);
        for (ground, bg) in grounds(theme) {
            let fg = eval("var(--accent-text)", theme, 0.0);
            check.pair(
                &format!("--accent-text on --accent-soft over {ground}"),
                theme,
                fg,
                accent_soft.over(bg),
                AA_TEXT,
            );
        }
    }
    check.done();
}

#[test]
fn hue_tokens_meet_aa_over_the_whole_hue_range() {
    let mut check = Check(Vec::new());
    for theme in THEMES {
        for h in 0..360 {
            let hue = f64::from(h);
            let at = |v: &str| eval(v, theme, hue);
            for (ground, bg) in grounds(theme) {
                // words in the hue: on the page, on a slot's wash, on a .badge.hue
                let wash = at("var(--hue-wash)");
                let badge = at("color-mix(in srgb, var(--hue-ink) 15%, transparent)").over(bg);
                let text = at("var(--hue-text)");
                check.pair(
                    &format!("--hue-text {h} on {ground}"),
                    theme,
                    text,
                    bg,
                    AA_TEXT,
                );
                check.pair(
                    &format!("--hue-text {h} on --hue-wash"),
                    theme,
                    text,
                    wash,
                    AA_TEXT,
                );
                check.pair(
                    &format!("--hue-text {h} on .badge.hue over {ground}"),
                    theme,
                    text,
                    badge,
                    AA_TEXT,
                );
                // a line, a bar or a dot in the hue
                check.pair(
                    &format!("--hue-ink {h} on {ground}"),
                    theme,
                    at("var(--hue-ink)"),
                    bg,
                    AA_LARGE_OR_LINE,
                );
            }
            // a slot: its title and its second line on its fill
            for fill in ["--hue-wash", "--hue-soft"] {
                let bg = at(&format!("var({fill})"));
                for text in ["--text", "--text-soft"] {
                    check.pair(
                        &format!("{text} on {fill} {h}"),
                        theme,
                        at(&format!("var({text})")),
                        bg,
                        AA_TEXT,
                    );
                }
            }
            // a tile icon's or login mark's letter: large text on its tint
            let mark_bg = at("var(--mark-bg)");
            let mark_ink = at("var(--mark-ink)");
            check.pair(
                &format!("--mark-ink on --mark-bg {h}"),
                theme,
                mark_ink,
                mark_bg,
                AA_LARGE_OR_LINE,
            );
        }
    }
    check.done();
}

#[test]
fn the_contrast_check_catches_a_failing_pair() {
    // the known WCAG figures: black on white 21:1, #767676 on white 4.54:1
    let white = Rgba(1.0, 1.0, 1.0, 1.0);
    assert!((contrast(Rgba(0.0, 0.0, 0.0, 1.0), white) - 21.0).abs() < 1e-9);
    let grey = eval("#767676", Theme::Light, 0.0);
    assert!((contrast(grey, white) - 4.54).abs() < 0.01);
    // the old light --hue-text at hue 60 fails on white
    let old = eval(
        "light-dark(hsl(var(--hue, 220) 55% 34%), #000000)",
        Theme::Light,
        60.0,
    );
    assert!(contrast(old, white) < AA_TEXT);
}
