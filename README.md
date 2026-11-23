# castle-design

The design system for my personal home apps: one look for every web app
behind the castle gate (the home dashboard, lights, maia's admin console,
llm-proxy's admin UI and the calendar). It can change any second; apps pin a tag.

What is here:

| path | what |
| --- | --- |
| [`assets/castle.css`](assets/castle.css) | the shared stylesheet: tokens, base, backdrop, shell, components |
| [`assets/castle.webp`](assets/castle.webp) | the castle picture behind every page (2400 × 1022) |
| [`assets/ircord-castle.ircord-theme.json`](assets/ircord-castle.ircord-theme.json) | the castle look as an [ircord](https://ircord.com) theme (see below) |
| [`docs/design-language.md`](docs/design-language.md) | the rules: tokens, themes, density, layout, components, touch |
| [`docs/mockup.html`](docs/mockup.html) | every component drawn on example screens, phone and desktop, light and dark (one self-contained file; open it in a browser) |
| [`src/lib.rs`](src/lib.rs) | the `castle-design` crate: the assets as compile-time constants and the theme cookie helper |

## Using it from an app

Add the crate as a git dependency, pinned to a tag:

```toml
[dependencies]
castle-design = { git = "https://github.com/ppetermann/castle-design", tag = "v0.1.10" }
```

The repository is public, so the fetch needs no token in CI or a Docker build.
The crate has no dependencies of its own.

Serve the assets from the app's own routes, then link the shared sheet before
the app's own one and point `--castle` at the picture:

```rust
use castle_design::{CASTLE_WEBP, CASTLE_WEBP_TYPE, CSS, VERSION};

// GET /static/castle.css  -> CSS          (text/css)
// GET /static/castle.webp -> CASTLE_WEBP  (CASTLE_WEBP_TYPE)
// <link rel="stylesheet" href="/static/castle.css?v={VERSION}">
// <link rel="stylesheet" href="/static/app.css">
// app.css: :root { --castle: url("/static/castle.webp"); }
```

An app that keeps its page in one string can inline `CSS` in a `<style>`
instead.

### The theme

One `theme` cookie on `Domain=my-castle.net` holds the choice for every app
(system, light or dark). `castle_design::Theme` reads and writes it the same
way everywhere:

```rust
use castle_design::Theme;

let theme = Theme::from_cookie_headers(cookie_header_values); // System without a cookie
let set_cookie: String = Theme::Dark.set_cookie();            // for the switch's response
// An app that once set a host-only `theme` cookie adds these to its pages:
let migrate: Vec<String> = Theme::legacy_cookie_migration(cookie_header_values);
// <html data-theme="{theme.data_theme()}">  (omit the attribute when None)
// <meta name="color-scheme" content="{theme.color_scheme()}">
```

A host-only fallback, where the shared cookie cannot be set, is named
`theme_local` (`Theme::set_local_cookie`), never `theme`, so the two never
collide; `from_cookie_headers` prefers `theme` and reads `theme_local` only
when `theme` is absent.

The stylesheet does the rest: every colour is a `light-dark()` token, so
`data-theme` on `<html>` flips the whole page. `data-density="dense"` on
`<html>` switches to the smaller scale for the admin screens.

### The shared icons

The home link's house and the theme switch's monitor, sun and moon are the
same in every app: `castle_design::icons::{HOME, MONITOR, SUN, MOON}` are
whole inline `<svg class="icon" aria-hidden="true">` strings to drop into the
app's own markup (the files are in `assets/icons/` too). The link and the
switch themselves, and the labels they carry, stay the app's:

```rust
use castle_design::icons;

let home = format!(r#"<a class="home-link" href="https://home.example.com/">{}<span>Home</span></a>"#, icons::HOME);
let light = format!(r#"<button name="theme" value="light" aria-label="Light" aria-pressed="false">{}</button>"#, icons::SUN);
```

## The castle in ircord

`assets/ircord-castle.ircord-theme.json` is an ircord personal theme file: the
castle's light and dark palettes as the theme's colours, and the shape tokens
(corners, system fonts) as its CSS. It uses ircord's theme support as it is;
nothing in either client changes.

Load it in either client (needs a signed-in account; the theme then lives in
the account and shows in both):

1. Settings → Appearance → *Your themes* → **Paste theme** (web) or **Import
   file / paste** (the desktop app), and paste the file's contents or choose it.
2. **Apply** the imported *Castle*, then pick *Light* or *Dark* under Theme
   (or *System*): the theme carries both.
3. Web client: **Edit** → *Background* → **Choose image** and pick
   [`assets/castle.webp`](assets/castle.webp). This is the step that makes it the
   castle: the theme is built around the picture, and its CSS turns the
   panels into frosted glass over it. The file already carries the background
   settings (whole app, bottom), but ircord never puts an image in a theme
   file, so it has to be chosen once per account. Without it the theme is
   the plain castle palette.

The desktop (GPUI) client does not show the picture yet; it will once its
theme background support lands. What it does not pick up either: the CSS beyond the root variables it reads
(colours, corners, interface scale; it leaves fonts to the app), so the
glass panels are web only. The castle apps' shadows and tab styling are not
part of a theme: ircord's own layout decides those. The colours are the castle's
`--bg`, `--surface`, `--text`, `--muted` and `--accent` per theme; the other
palette hues (cyan, yellow, pink) are not castle tokens and are chosen to sit
with them.

## Pinning and updating

Pin a **tag** (`tag = "v0.2.0"`). `Cargo.lock` then records an exact commit
and the [changelog](CHANGELOG.md) says what that version changed. To move an
app to a new version, bump the tag in its `Cargo.toml` and run:

```sh
cargo update -p castle-design
```

A `branch = "main"` pin also works (the same `cargo update -p castle-design`
moves it to the branch's current commit), but it gives no name to what
changed; use it only while trying something out.

Token names are a contract: adding one is free, renaming or removing one is a
breaking change and gets a new minor version while the crate is 0.x.

## Development

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

The tests hold the stylesheet to the design language's rules: no
`prefers-color-scheme` block, no colour outside a token, every token it
reads declared (or listed as an app token), no mockup scaffolding left.

## License

MIT, see [LICENSE](LICENSE).
