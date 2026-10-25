# Changelog

Each release is a tag (`vX.Y.Z`) that apps pin. While the crate is 0.x, a
renamed or removed token, class or public item bumps the minor version.

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
