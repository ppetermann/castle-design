//! The ircord theme file keeps to the stylesheet's palette: its seeds are
//! castle.css's tokens, so a change of colour there fails here until the
//! theme follows.

const THEME: &str = include_str!("../assets/ircord-castle.ircord-theme.json");
const CSS: &str = castle_design::CSS;

#[test]
fn theme_seeds_are_castle_tokens() {
    // (token's light-dark pair in castle.css, the seed values in the theme)
    for hex in [
        "#07090d", "#161a22", "#e8ebf1", "#9aa3b2", "#7ea6ff", "#e4e8ef", "#ffffff", "#141821",
        "#5a6473", "#2b59c3",
    ] {
        assert!(THEME.contains(hex), "theme lacks {hex}");
        assert!(CSS.contains(hex), "castle.css no longer has {hex}");
    }
}

#[test]
fn theme_file_is_an_ircord_theme_with_both_modes() {
    assert!(THEME.contains("\"ircordTheme\": 1"));
    assert!(THEME.contains("\"dark\": {\n      \"canvas\""));
    assert!(THEME.contains("\"light\": {\n      \"canvas\""));
    assert!(
        !THEME.contains("url("),
        "ircord strips url() from theme CSS"
    );
}
