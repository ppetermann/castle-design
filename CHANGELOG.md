# Changelog

Each release is a tag (`vX.Y.Z`) that apps pin. While the crate is 0.x, a
renamed or removed token, class or public item bumps the minor version.

## 0.1.4

### Added

Everything here is opt-in: a page that uses none of the new classes looks as
it did in 0.1.3.

- Hue tokens: `--hue-ink`, `--hue-text`, `--hue-wash` and `--hue-soft`, on
  `:root` and re-derived on any element that sets `--hue` inline; `.swatch.hue`
  (a dot in the hue, `.sm`/`.lg`) and `.badge.hue`.
- Toggle chips (`.chips.toggle`): a link or button with `aria-pressed`, a
  `.count`, an optional always-visible second target (`.act`), the off state
  dashed with a hollow swatch, `li.plain` for "show all"; 44 px under a coarse
  pointer. `.chips.scroll` keeps chips on one swipeable row on a phone.
- Segmented: links (`aria-current`) and radio labels as choices, `.sm`, and
  `.wrap`, `.fill` and `.fill-phone`, which wrap or split into equal columns
  without losing a hairline.
- Pill checks (`.pills`, `.check.pill`), a swatch picker (`.swatches`) and
  `fieldset.group` (a legend as a field label, no box).
- The schedule list (`ul.schedule` with `li.day`, `.past`, `.now`, `.next`,
  `li.more`) and the slot (`.slot`, `.row`, `.auto` with a bolt, `.more`).
- The time grid (`.timegrid` with `.tg-head`, `.tg-allday`, `.tg-body`,
  `.tg-hours`, `.tg-col`, `.tg-now`): columns against an hours axis, slots
  placed by `--start`/`--len` (and `--lane`/`--lanes`), the now line at `--at`
  in the new `--now` token.
- `.card.flush`; `.card.dialog` for a confirm (native `<dialog>` or scripted);
  `.sheet-head .close`, `.sheet-body.flat` for a page fetched into a sheet,
  and `.sheet.docked`, a non-modal sheet under the bar beside a live page.
- The design language documents all of it, the calendar as a castle app and
  its phone primary action as an `.icon-btn` "+" in the bar; the mockup gains
  the calendar's week, docked editor and agenda.

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
