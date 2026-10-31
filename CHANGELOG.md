# Changelog

Each release is a tag (`vX.Y.Z`) that apps pin. While the crate is 0.x, a
renamed or removed token, class or public item bumps the minor version.

## 0.1.8

### Added

Everything here is opt-in: a page that uses none of the new classes looks as
it did in 0.1.7.

- The diff (`pre.diff`): a unified diff with one `span` per line, `.add` in
  `--ok-text`, `.del` in `--err-text`, `.hunk` in `--accent-text` and `.meta`
  muted. `.diffhead` names the change in one mono line (`.path`, `.plus` and
  `.minus` counts, a link), and `details.diffblock` folds the `pre` under it
  (18rem at most, its summary at least `--tap` high). An approval's preview of a
  diff now needs no rules of its own.
- Stats with a tone: `.stat.ok`, `.stat.warn` and `.stat.err` colour the
  value in their `-text` token; `.stat.flat` is a cell on `--surface-2`
  without glass, for a panel that is glass already; `.stats.inchat` fits a
  strip into a chat bubble, two cells a row on a phone, with a `.caption`
  spanning the row.
- The wait (`.note.wait`): a note with an accent bar, a pulsing `.dot` and a
  mono `.countdown` the page counts down; `.due` once the time has passed
  mutes it and stops the dot. The pulse is `@keyframes castle-pulse`, off
  under `prefers-reduced-motion`.
- Link chips (`.chips.links`): each pill holds one `<a>` that fills it
  (`--tap` high under a finger) with a mono `.glyph` for its kind;
  `li.refused` is a link the app will not open, dashed, its `.host` in mono.
- `.dialog .address`: a confirm about a link shows its whole address in a
  mono line on `--bg-sunken`.
- `.choice .reply` with a `textarea.input`: its button sits at the box's
  foot, for an answer of several lines; the mockup draws one.
- `tests/contrast.rs` holds a diff's line colours and the code words on
  `--bg-sunken`, a wait's countdown and bar, and a toned stat on a flat cell
  to AA in both themes.

### Fixed

- An approval's `.preview` summary is at least `--tap` high: its padding was
  worked out from a font larger than its own, so it stood about 32px tall under
  a 40px `--tap`. It is a flex row now, with its own ▸/▾ mark in place of the
  browser's.

## 0.1.7

### Added

Everything here is opt-in: a page that uses none of the new classes looks as
it did in 0.1.6.

- The approval (`fieldset.choice.approval`): a choice that asks leave for one
  step that cannot be undone. A mono `.kind` word in `--ask-text` before the
  question, `ul.what` with what will happen, `details.preview` folding the
  command or diff in a `pre` (18rem at most), and Approve (`.btn.primary`),
  Deny… (`.btn.ghost.deny`, in err words) and an optional Edit…
  (`.btn.ghost`) in `.options`. Answered, the chosen button
  (`aria-pressed="true"`) stays at full strength while the others fade.
- `.choice .reply`: a row under a choice's options for an answer in words
  (an approval's reason or edited value): an `.input` or `textarea.input`,
  its button and a `.hint`; `hidden` hides it.
- The checklist's states and a plan's head: `.n.failed` (an `!` in
  `--err-text` on an err wash, rimmed in `--err`), `.n.skipped` (dashed) with
  `li.skipped` (its label struck through and muted), the current step's label
  in `--accent-text` on `li.cur`, a `.step-note` under a label, `li.head` with
  the list's title and a `.count`, and `.checklist.compact` for a chat bubble.
- `details.superseded`: an earlier copy of a list folded to one dashed mono
  line, with a link to the latest and the old list faded inside.
- `tests/contrast.rs` holds the failed step's mark to AA on its wash and its
  rim to 3:1 on every surface, reading both from the rule. (White on the vivid
  `--err` fill would reach only 2.77:1 in the dark theme, so the failed disc is
  a wash, like a badge.)
- The design language documents the approval, the reply row and the
  checklist's states, and the touch table their targets; the mockup's
  foundations screen draws an open and an answered approval and a plan's
  checklist with a superseded copy above it.

## 0.1.6

### Added

Everything here is opt-in: a page that uses none of the new classes looks as
it did in 0.1.5.

- `--ask` and `--ask-text`, for a question waiting for a person, in both
  themes; `--ask-text` reaches AA on every surface and on its badge wash,
  `--ask` 3:1 as a bar (`tests/contrast.rs`).
- The state card (`article.state-card`): a title, a state badge, an optional
  task id and a few lines, its bar, dot and badge coloured by the state class
  (`ok`, `done`, `working`, `waiting`, `needs-decision`, `blocked`,
  `failed`; any other muted).
- The choice form (`fieldset.choice`): a prompt, options as `.btn.option`
  buttons (`aria-pressed`) or labels holding a hidden radio or checkbox, a
  description line per option (`.about`), a submit for several picks and the
  answer (`.answered`); answered, the fieldset is disabled and the chosen
  options stay at full strength.
- The grid table (`.grid-table`): every cell ruled, numbers tabular, a
  caption, the table as wide as its cells and scrolling sideways in its box
  on a phone.
- The design language documents the three and the mockup's foundations
  screen draws them. They are what an app's chat draws for an agent's OpenUI
  blocks (a status, a table, a question).
- An app that keeps its own copies of these moves onto them by: naming the
  choice's option row `.options`; putting the state's own word on the card
  (`working`, `needs-decision`, …) instead of a colour name, since the sheet
  maps the seven states itself; and dropping its own `--ask` and
  `--ask-text` and any dot or badge it drew in them.

### Fixed

- The mockup's copy of `.btn` no longer outranks `.btn.primary` and the
  other variants, so its primary and danger buttons are drawn as castle.css
  draws them (the sheet itself was right).

## 0.1.5

### Fixed

- `.segmented` and `.pager .pages` no longer style a link marked
  `aria-current="false"` as the current choice; any other `aria-current`
  value still marks it.
- The light `--hue-ink` is a little darker (`hsl(h 60% 34%)`, was 35%), so a
  `.slot`'s bar reaches 3:1 on its own `--hue-wash` at every hue; it dipped to
  2.91:1 around hue 60. The washes and `--hue-soft` follow it.

## 0.1.4

### Added

Everything under Added is opt-in: a page that uses none of the new classes
looks as it did in 0.1.3, apart from the two darker status words under Changed.

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
- `tests/contrast.rs`: WCAG AA computed from the sheet, for every text token
  on every surface and badge wash, and for the hue tokens at every hue in
  both themes (4.5:1 for words, 3:1 for lines and large text).

### Changed

- Light `--ok-text` `#15803d` → `#137236` and `--warn-text` `#b45309` →
  `#9e4908`: both were just under 4.5:1 on `--bg` and on their own badge
  wash. Slightly darker words in ok and warn badges and notes; names unchanged.

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
