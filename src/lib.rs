//! The castle's design system, shared by its web apps (home, lights, maia's
//! admin console, llm-proxy's admin UI and the calendar).
//!
//! The assets are compiled in: an app serves [`CSS`] and [`CASTLE_WEBP`] from
//! its own routes (or inlines the CSS), so a build needs no network beyond the
//! git fetch of this crate and no app depends on another being up. The rules
//! behind them are in `docs/design-language.md`.
//!
//! [`Theme`] reads and writes the one `theme` cookie every app shares on
//! [`THEME_COOKIE_DOMAIN`], so a choice made in one app's switch holds in all
//! of them. A host-only fallback is named [`THEME_LOCAL_COOKIE`], never
//! `theme`, so the two can never be mistaken for each other.

/// The shared stylesheet: tokens, base, backdrop, shell and components.
pub const CSS: &str = include_str!("../assets/castle.css");

/// The castle picture behind every page (WebP, 2400 × 1022). An app serves it
/// and points the stylesheet's `--castle` token at that URL.
pub const CASTLE_WEBP: &[u8] = include_bytes!("../assets/castle.webp");

/// The picture's media type, for the `Content-Type` an app serves it with.
pub const CASTLE_WEBP_TYPE: &str = "image/webp";

/// This crate's version, for a cache-busting query on the asset URLs
/// (`/static/castle.css?v=0.1.3`).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The theme cookie's name.
pub const THEME_COOKIE: &str = "theme";

/// The name of a host-only fallback theme cookie, for an app served off
/// [`THEME_COOKIE_DOMAIN`] or one that cannot set the shared cookie. Read only
/// when no [`THEME_COOKIE`] is present.
pub const THEME_LOCAL_COOKIE: &str = "theme_local";

/// The domain the theme cookie is set on, so every app under it shares one
/// choice.
pub const THEME_COOKIE_DOMAIN: &str = "my-castle.net";

/// Whether `host` (a `Host` header or URL host, with or without a port) is
/// [`THEME_COOKIE_DOMAIN`] or a subdomain of it, so the shared theme cookie
/// reaches it. Host names are case-insensitive and may end in a root dot, so
/// `Lights.My-Castle.NET.` counts; a lookalike such as `evilmy-castle.net`
/// does not, nor does a host with an empty label (`.my-castle.net`,
/// `evil..my-castle.net`).
pub fn is_castle_host(host: &str) -> bool {
    let host = match host.rsplit_once(':') {
        Some((name, port)) if !name.contains(':') && port.bytes().all(|b| b.is_ascii_digit()) => {
            name
        }
        _ => host,
    };
    let host = host.strip_suffix('.').unwrap_or(host);
    if host.split('.').any(str::is_empty) {
        return false;
    }
    let host = host.as_bytes();
    let domain = THEME_COOKIE_DOMAIN.as_bytes();
    match host.len().checked_sub(domain.len()) {
        Some(0) => host.eq_ignore_ascii_case(domain),
        Some(at) => host[at - 1] == b'.' && host[at..].eq_ignore_ascii_case(domain),
        None => false,
    }
}

/// How long a chosen theme is kept: a year, in seconds.
pub const THEME_COOKIE_MAX_AGE: u32 = 365 * 24 * 60 * 60;

/// The colours a page is drawn in: the browser's preference unless the theme
/// cookie names one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    System,
    Light,
    Dark,
}

impl Theme {
    /// Every theme, in the order the switch shows them.
    pub const ALL: [Theme; 3] = [Theme::System, Theme::Light, Theme::Dark];

    /// The theme's name, as the cookie and the switch's form carry it.
    pub fn as_str(self) -> &'static str {
        match self {
            Theme::System => "system",
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    /// The theme a name stands for, if it is one of the three.
    pub fn parse(value: &str) -> Option<Self> {
        Theme::ALL.into_iter().find(|t| t.as_str() == value)
    }

    /// The theme a request's `Cookie` header values ask for; system without a
    /// valid theme cookie. Pass every `Cookie` header the request carries.
    ///
    /// The shared `theme` cookie wins over a `theme_local` fallback wherever
    /// either sits in the headers. When `theme` appears twice (a legacy
    /// host-only one beside the shared one) the last valid value wins: browsers
    /// send cookies of the same path oldest first, and the legacy one is the
    /// older; [`Theme::legacy_cookie_migration`] removes it.
    pub fn from_cookie_headers<'a>(headers: impl IntoIterator<Item = &'a str>) -> Self {
        let headers: Vec<&str> = headers.into_iter().collect();
        let read = |name| {
            let mut values = cookie_values(headers.iter().copied(), name).peekable();
            values
                .peek()
                .is_some()
                .then(|| values.filter_map(Theme::parse).last())
        };
        read(THEME_COOKIE)
            .or_else(|| read(THEME_LOCAL_COOKIE))
            .flatten()
            .unwrap_or_default()
    }

    /// The `Set-Cookie` values that move a legacy host-only `theme` cookie onto
    /// the shared one; empty when the request carries no `theme` cookie. Pass
    /// every `Cookie` header the request carries.
    ///
    /// A request does not say which cookie is host-only, so: with one `theme`
    /// cookie its value is written to the shared cookie (a no-op when it was
    /// the shared one already); with two the shared one exists and keeps its
    /// value. Either way the host-only `theme` is expired (`Max-Age=0` without
    /// `Domain`, which leaves the shared cookie alone). Only an app that once
    /// set a host-only `theme` needs to call this, on its page responses.
    pub fn legacy_cookie_migration<'a>(headers: impl IntoIterator<Item = &'a str>) -> Vec<String> {
        let values: Vec<&str> = cookie_values(headers, THEME_COOKIE).collect();
        let mut set_cookies = match values.as_slice() {
            [] => return Vec::new(),
            [value] => Theme::parse(value)
                .filter(|t| *t != Theme::System)
                .map(Theme::set_cookie)
                .into_iter()
                .collect(),
            _ => Vec::new(),
        };
        set_cookies.push(format!(
            "{THEME_COOKIE}=; Path=/; Max-Age=0; SameSite=Lax; Secure"
        ));
        set_cookies
    }

    /// The `Set-Cookie` value that keeps this theme for a year on the whole
    /// castle; system clears the cookie. Not `HttpOnly`: it carries no secret
    /// and a page that renders in the browser reads it before paint.
    pub fn set_cookie(self) -> String {
        let (value, max_age) = match self {
            Theme::System => ("", 0),
            theme => (theme.as_str(), THEME_COOKIE_MAX_AGE),
        };
        format!(
            "{THEME_COOKIE}={value}; Domain={THEME_COOKIE_DOMAIN}; Path=/; Max-Age={max_age}; SameSite=Lax; Secure"
        )
    }

    /// The `Set-Cookie` value for the host-only `theme_local` fallback, for an
    /// app that cannot set the shared cookie; system clears it.
    pub fn set_local_cookie(self) -> String {
        let (value, max_age) = match self {
            Theme::System => ("", 0),
            theme => (theme.as_str(), THEME_COOKIE_MAX_AGE),
        };
        format!("{THEME_LOCAL_COOKIE}={value}; Path=/; Max-Age={max_age}; SameSite=Lax; Secure")
    }

    /// The html element's `data-theme`, which overrides the system's choice;
    /// none for system.
    pub fn data_theme(self) -> Option<&'static str> {
        (self != Theme::System).then(|| self.as_str())
    }

    /// The `<meta name="color-scheme">` content for this theme.
    pub fn color_scheme(self) -> &'static str {
        match self {
            Theme::System => "light dark",
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }
}

/// The values of every cookie called `name` in the `Cookie` header values, in
/// the order the browser sent them.
fn cookie_values<'a>(
    headers: impl IntoIterator<Item = &'a str>,
    name: &'a str,
) -> impl Iterator<Item = &'a str> {
    headers
        .into_iter()
        .flat_map(|v| v.split(';'))
        .filter_map(|pair| pair.trim().split_once('='))
        .filter(move |(n, _)| *n == name)
        .map(|(_, value)| value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_theme_cookie_among_others() {
        assert_eq!(
            Theme::from_cookie_headers(["session=abc; theme=dark; other=1"]),
            Theme::Dark
        );
        assert_eq!(
            Theme::from_cookie_headers(["session=abc", "theme=light"]),
            Theme::Light
        );
    }

    #[test]
    fn falls_back_to_system() {
        assert_eq!(Theme::from_cookie_headers([]), Theme::System);
        assert_eq!(Theme::from_cookie_headers(["theme=purple"]), Theme::System);
        assert_eq!(Theme::from_cookie_headers(["themes=dark"]), Theme::System);
    }

    #[test]
    fn prefers_the_shared_cookie_over_the_local_one() {
        assert_eq!(
            Theme::from_cookie_headers(["theme_local=light; theme=dark"]),
            Theme::Dark
        );
        assert_eq!(
            Theme::from_cookie_headers(["theme=dark", "theme_local=light"]),
            Theme::Dark
        );
        assert_eq!(
            Theme::from_cookie_headers(["session=abc; theme_local=light"]),
            Theme::Light
        );
    }

    #[test]
    fn an_invalid_shared_cookie_blocks_the_local_one() {
        assert_eq!(
            Theme::from_cookie_headers(["theme=purple; theme_local=dark"]),
            Theme::System
        );
        assert_eq!(
            Theme::from_cookie_headers(["theme_local=dark", "theme="]),
            Theme::System
        );
    }

    #[test]
    fn recognises_castle_hosts_in_any_case() {
        for host in [
            "my-castle.net",
            "lights.my-castle.net",
            "Lights.My-Castle.NET",
            "MY-CASTLE.NET.",
            "home.my-castle.net.:8443",
            "home.my-castle.net:443",
        ] {
            assert!(is_castle_host(host), "{host}");
        }
        for host in [
            "evilmy-castle.net",
            "EvilMy-Castle.net",
            "my-castle.net.evil.example",
            "my-castle.network",
            "net",
            "",
            ".",
            "localhost:8080",
            ".my-castle.net",
            "evil..my-castle.net",
            "my-castle.net..",
            "lights.my-castle..net",
        ] {
            assert!(!is_castle_host(host), "{host}");
        }
    }

    #[test]
    fn a_legacy_host_only_cookie_does_not_win() {
        // Same path, so the browser sends the older (legacy host-only) first.
        assert_eq!(
            Theme::from_cookie_headers(["theme=light; theme=dark"]),
            Theme::Dark
        );
        assert_eq!(
            Theme::from_cookie_headers(["theme=light; theme=purple"]),
            Theme::Light
        );
    }

    #[test]
    fn migrates_a_legacy_host_only_cookie() {
        let expire = "theme=; Path=/; Max-Age=0; SameSite=Lax; Secure";
        assert!(Theme::legacy_cookie_migration(["session=abc; theme_local=dark"]).is_empty());
        // Alone, its value moves to the shared cookie.
        assert_eq!(
            Theme::legacy_cookie_migration(["session=abc; theme=dark"]),
            [Theme::Dark.set_cookie(), expire.to_string()]
        );
        // Beside the shared cookie, which keeps its own value.
        assert_eq!(
            Theme::legacy_cookie_migration(["theme=light; theme=dark"]),
            [expire]
        );
        assert_eq!(Theme::legacy_cookie_migration(["theme=purple"]), [expire]);
        assert!(!expire.contains("Domain"));
    }

    #[test]
    fn set_local_cookie_stays_on_the_host() {
        assert_eq!(
            Theme::Light.set_local_cookie(),
            "theme_local=light; Path=/; Max-Age=31536000; SameSite=Lax; Secure"
        );
        assert!(
            Theme::System
                .set_local_cookie()
                .starts_with("theme_local=; ")
        );
        assert!(Theme::System.set_local_cookie().contains("Max-Age=0;"));
    }

    #[test]
    fn set_cookie_covers_the_castle() {
        assert_eq!(
            Theme::Dark.set_cookie(),
            "theme=dark; Domain=my-castle.net; Path=/; Max-Age=31536000; SameSite=Lax; Secure"
        );
        assert!(Theme::System.set_cookie().starts_with("theme=; "));
        assert!(Theme::System.set_cookie().contains("Max-Age=0;"));
        assert!(!Theme::Light.set_cookie().contains("HttpOnly"));
    }

    #[test]
    fn names_round_trip() {
        for theme in Theme::ALL {
            assert_eq!(Theme::parse(theme.as_str()), Some(theme));
        }
        assert_eq!(Theme::System.data_theme(), None);
        assert_eq!(Theme::Light.data_theme(), Some("light"));
    }
}
