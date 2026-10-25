# castle-design

The design system for my personal home apps: one look for every web app
behind the castle gate (the home dashboard, lights, maia's admin console and
llm-proxy's admin UI). It can change any second; apps pin a tag.

What is here:

| path | what |
| --- | --- |
| [`assets/castle.css`](assets/castle.css) | the shared stylesheet: tokens, base, backdrop, shell, components |
| [`assets/castle.webp`](assets/castle.webp) | the castle picture behind every page (2400 × 1022) |
| [`docs/design-language.md`](docs/design-language.md) | the rules: tokens, themes, density, layout, components, touch |
| [`docs/mockup.html`](docs/mockup.html) | every component drawn on example screens, phone and desktop, light and dark (one self-contained file; open it in a browser) |
| [`src/lib.rs`](src/lib.rs) | the `castle-design` crate: the assets as compile-time constants and the theme cookie helper |

## Using it from an app

Add the crate as a git dependency, pinned to a tag:

```toml
[dependencies]
castle-design = { git = "https://github.com/ppetermann/castle-design", tag = "v0.1.3" }
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
