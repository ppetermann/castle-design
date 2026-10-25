# Changelog

Each release is a tag (`vX.Y.Z`) that apps pin. While the crate is 0.x, a
renamed or removed token, class or public item bumps the minor version.

## 0.1.3

### Added

- `is_castle_host`: whether a host is on the castle domain (so the shared
  theme cookie reaches it), compared ASCII case-insensitively, tolerant of a
  trailing root dot and a port, and rejecting lookalikes such as
  `evilmy-castle.net` or hosts with an empty label. New in this release;
  each app adopts it in its own update, replacing its case-sensitive check.

### Fixed

- A test now pins that a present but invalid `theme` cookie blocks the
  `theme_local` fallback (system wins), as documented.

## 0.1.2

### Added

- `THEME_LOCAL_COOKIE` (`theme_local`): the name of a host-only fallback theme
  cookie, for an app off the castle domain or one that cannot set the shared
  cookie, and `Theme::set_local_cookie` to write it.
- `Theme::legacy_cookie_migration`: the `Set-Cookie` values that move a legacy
  host-only `theme` cookie onto the shared one and expire it.

### Fixed

- `Theme::from_cookie_headers` no longer lets a legacy host-only `theme`
  cookie win over the shared one: the shared `theme` is preferred over
  `theme_local` wherever either sits in the headers, and of two `theme`
  cookies the last valid one (the newer, shared one) wins.

## 0.1.1

### Fixed

- On a phone `.popover` spans the screen under the top bar again: its phone
  rule sat before the base rule, so the base `position: absolute` won and the
  panel ran off the right edge. The ultra-wide `.card-grid` override moved
  after its base rule too, and a test keeps every `@media` override after the
  rule it overrides.

## 0.1.0

The first version.

### Added

- `assets/castle.css`: the shared stylesheet (tokens written once with
  `light-dark()`, comfortable and dense scales, backdrop, shell, surfaces,
  status, controls, lists and tables, stats, lamp tiles, login, empty state,
  pager, code and logs).
- `assets/castle.webp`: the castle picture.
- `docs/design-language.md` and `docs/mockup.html`.
- The `castle-design` crate: `CSS`, `CASTLE_WEBP`, `CASTLE_WEBP_TYPE`,
  `VERSION`, and `Theme` with the shared `theme` cookie on `my-castle.net`
  (`THEME_COOKIE`, `THEME_COOKIE_DOMAIN`, `THEME_COOKIE_MAX_AGE`).
