//! The castle's design system, shared by its web apps (home, lights, maia's
//! admin console, llm-proxy's admin UI).
//!
//! The assets are compiled in: an app serves [`CSS`] and [`CASTLE_WEBP`] from
//! its own routes (or inlines the CSS), so a build needs no network beyond the
//! git fetch of this crate and no app depends on another being up. The rules
//! behind them are in `docs/design-language.md`.
//!
//! [`Theme`] reads and writes the one `theme` cookie every app shares on
//! [`THEME_COOKIE_DOMAIN`], so a choice made in one app's switch holds in all
//! of them.

/// The shared stylesheet: tokens, base, backdrop, shell and components.
pub const CSS: &str = include_str!("../assets/castle.css");

/// The castle picture behind every page (WebP, 2400 × 1022). An app serves it
/// and points the stylesheet's `--castle` token at that URL.
pub const CASTLE_WEBP: &[u8] = include_bytes!("../assets/castle.webp");

/// The picture's media type, for the `Content-Type` an app serves it with.
pub const CASTLE_WEBP_TYPE: &str = "image/webp";

/// This crate's version, for a cache-busting query on the asset URLs
/// (`/static/castle.css?v=0.1.1`).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The theme cookie's name.
pub const THEME_COOKIE: &str = "theme";

/// The domain the theme cookie is set on, so every app under it shares one
/// choice.
pub const THEME_COOKIE_DOMAIN: &str = "my-castle.net";

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
    pub fn from_cookie_headers<'a>(headers: impl IntoIterator<Item = &'a str>) -> Self {
        headers
            .into_iter()
            .flat_map(|v| v.split(';'))
            .filter_map(|pair| pair.trim().split_once('='))
            .find(|(name, _)| *name == THEME_COOKIE)
            .and_then(|(_, value)| Theme::parse(value))
            .unwrap_or_default()
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
