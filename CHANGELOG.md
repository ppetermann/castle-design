# Changelog

Each release is a tag (`vX.Y.Z`) that apps pin. While the crate is 0.x, a
renamed or removed token, class or public item bumps the minor version.

## 0.1.10

A sweep of the design review's small findings. Pages change in a few places
without any markup change; they are listed first.

### Changed

- `--on-fill` is `#0b1020` in the dark theme (white stays in light): white on
  the dark theme's pale `--err` and `--ok` reached only 2.8:1 and 2.3:1, so a
  tab's count, a sidebar badge, a danger button and a done step's ✓ were
  under AA there. It now works like `--accent-contrast`.
- `--surface-2` is `#f5f7fa` in light (it was `#eef1f6`, the same as `--bg`):
  a state card, a choice form, a chip or a switch's track placed straight on
  the page showed no edge. It now sits between the page and a card, so it
  shows on both.
- A placeholder is `--muted` at full strength (it was faded to .8, 3.8:1 in
  light).
- Where the soft `--ring` was the only sign of focus (a pill check, a
  swatch, a choice's option label, a segmented choice) a solid 2 px accent
  ring takes its place, as everywhere else; a segmented control's buttons
  and links get it inside their edge, since the box clips an outline. An
  `.input` keeps `--ring` beside its accent border.
- A lamp's state (`.lamp .status`) is no longer faded to .85: its ink is
  picked for contrast against the lamp's own colour.
- Under a coarse pointer an `.input`'s text is at least 16 px, so iOS no
  longer zooms the page on focus in a dense app.
- The ultra-wide rule reaches sidebar apps: `.with-sidenav main.page` grows
  to `max(--page, 80vw)` plus the sidebar, as the design language says.
- The base layer: `h4`–`h6`, `ul`, `ol`, `dl` and `blockquote` lose the
  browser's margins for `--s3` under them, like `p` (`h4`–`h6` at `--t-s`
  600); lists indent 1.25em; `hr` is a `--line` hairline; `blockquote` a
  `--line-strong` bar; `summary` takes the pointer; a link's underline
  thickens under the pointer.
- Every small uppercase label is tracked `--track-caps` (.05em); they were
  .04, .05 and .06em.
- The tile and the lamp tile are on the spacing and type scale (`--s*`,
  `--t-*`), so density reaches them; their sizes move by a pixel or two.
- Every z-index of the shell and the overlays reads a `--z-*` token; the
  values are unchanged.

### Added

- `.scrim.above`: a confirm's scrim over a sheet and under the confirm, for
  a confirm opened from a sheet (the plain scrim sits under the sheet).
- A docked sheet makes room: on a desk the page pads its right side by
  `--sheet-w` while a `.sheet.docked` is shown (not `hidden`), so the sheet
  sits beside a week's last day instead of over it.
- Status tones fill the grid: `.dot.info`, `.dot.ask`, `.badge.ask`,
  `.note.info`, `.note.ask`, `.toast.warn`, `.toast.info`, `.toast.ask`, and
  `--info-text` (the same value as `--info`) for words; `nav.tabs
  .count.ask` and `.nav-badge.ask` count things waiting on a person apart
  from failures.
- Form states: `.input:disabled`, `.input[readonly]`,
  `.input[aria-invalid="true"]` with a `.field .error` line, a `.field .req`
  required mark, and `select.input`.
- Progress and loading: `progress.progress` (determinate and
  indeterminate), `.spinner`, `.htmx-indicator` and a progress cursor on
  `[aria-busy="true"]`; `@keyframes castle-spin` and `castle-slide`.
- Forced colours and print (section 15): high-contrast mode puts chosen
  choices, pressed buttons, switches and focus rings back in system colours
  and hides the castle; print uses the light theme on white without the
  castle, glass, bars or sheets, the page at the paper's width, statuses and
  hues in colour.
- Tokens: `--sheet-w`, `--track-caps`, `--info-text` and the `--z-*` stack.
- Tests: `tests/contrast.rs` holds `--on-fill` on every fill it is drawn on,
  the placeholder in its field, every solid focus ring (and that `--ring`
  never shows focus alone), the progress bar on its track, `--surface-2`
  apart from the page and a card, and a lamp's unfaded state;
  `tests/assets.rs` holds the stacking order, the ultra-wide sidebar page,
  the docked sheet's room, the status tones, the uppercase tracking, the
  tile and lamp type scale, the iOS input size, the form states, and that
  every mockup rule lists the same selectors as castle.css's.

### Fixed

- The mockup's `.input` rule no longer carries `.screen select.input` and
  `.screen textarea.input`, which outranked `.input:focus` and kept a
  focused select's border neutral. The mockup also takes `--on-fill`
  instead of a literal white.
- The design language: badges, table heads, field labels and sidebar heads
  are 13 px (12 px dense), not 12 px; `--surface-2`'s uses; the touch table
  names the theme switch and inputs; the touch debt list names every
  undersized control (`.theme-switch` buttons, `.btn.link`, a plain chip's
  remove button and a `.close` outside a sheet join `.btn.sm` and
  `.icon-btn`); the chat-only components say they have no app yet.

## 0.1.9

### Added

Everything here is opt-in: a page that uses none of the new classes looks as
it did in 0.1.8.

- The stepper (`ol.stepper`): a thing's steps on one line, a disc per step
  with its name under it, joined by a hairline. A step is `.done` (filled in
  `--text-soft`), `.here` (larger, in its tone: accent by default, `.ok`,
  `.ask`, `.warn` or `.err`, its name bold) or still ahead (hollow), and the
  stretch after a `.done` step — the covered ground up to the current one —
  is darker. Say which step is current without the colour too: the app puts
  `aria-current="step"` on the here step and names the state in the list's
  accessible name. An agent's change or task in an app's chat (OpenUI).

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

Almost everything here is opt-in: a page that uses none of the new classes
looks as it did in 0.1.6, with one exception: a chosen `.btn` with
`aria-pressed="true"` in the `.options` of a disabled `fieldset.choice` no
longer fades (it stays at full strength, as an answered choice's chosen
option should).

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
