# The castle's design language

One look for every web app behind the castle gate: **home** (the dashboard at
home.my-castle.net), **lights** (z2m-mcp's web UI), **maia**'s admin console,
**llm-proxy**'s admin UI and the **calendar**. Home is the reference: a faded castle behind the
page, see-through tiles over it, a system/light/dark switch, calm type. The
other apps keep their structure and behaviour and take this look.

This repository is the single source: `assets/castle.css` is the shared
stylesheet, `assets/castle.webp` the picture, and the `castle-design` crate
hands both to each app at compile time (section 9). The mockup beside this
document, `mockup.html`, draws every screen with a frame-scoped copy of the
stylesheet. Where the mockup, the stylesheet and this text disagree, the text
is the intent.

Contents: 1 principles · 2 tokens · 3 themes · 4 density · 5 layout and
responsive rules · 6 components · 7 touch · 8 one-app elements · 9 where this
lives and how apps pick up changes · 10 what each app changes.

## 1. Principles

1. **The castle is the ground.** Every page stands on the same faded picture,
   fading in from the top so the bar and the title sit on plain background.
   Dense screens show it fainter; readability wins over atmosphere.
2. **Glass in three strengths.** A tile you tap is light glass (46 %, blur
   2 px). A panel that holds content is strong glass (80 %, blur 12 px). A
   table lives on an opaque surface. Nothing else is translucent.
3. **One scale, two densities.** Home and lights are read at arm's length and
   touched; maia and llm-proxy are worked at a desk with a mouse. Same tokens,
   same components, a smaller type scale and shorter controls under
   `data-density="dense"`.
4. **State is never colour alone.** A dot has a title, a badge has words, a
   lamp tile says "on · 40 %" beside its colour.
5. **Behaviour stays.** This document changes how things look and where they
   sit. Routes, forms, htmx polls, remote APIs and keyboard behaviour are out
   of scope; an app's AGENTS.md still owns them.
6. **No literal colours outside the tokens.** An app's stylesheet reads
   `var(--…)`. `castle.css` is held to this by its tests (a colour appears
   only in a custom property's declaration); each app should test its own
   sheet the same way. The one exception is forced colours (section 5),
   where the sheet names the user's own system colours (`Highlight`,
   `HighlightText`), never one of ours.

## 2. Tokens

The shared file declares these on `:root`. Colours are written once for both
themes with `light-dark()`; the theme is chosen by `color-scheme` (section 3).
Besides the tables below it declares `--on-fill` (the words on a vivid status
fill: a count, a danger button, a done step; white in light, `#0b1020` in dark,
where the fills are pale, as `--accent-contrast` is on the accent),
`--art-fade` (the mask that fades the castle in) and `--castle`, which is
`none` until the app points it at the URL it serves the picture from:
`:root { --castle: url("/static/castle.webp"); }`.

### Surfaces

| token | light | dark | for |
| --- | --- | --- | --- |
| `--bg` | `#eef1f6` | `#0c0f15` | the page under the picture |
| `--bg-sunken` | `#e4e8ef` | `#07090d` | code, logs, a recessed well |
| `--surface` | `#ffffff` | `#161a22` | inputs, opaque cards, a table's home |
| `--surface-2` | `#f5f7fa` | `#1e232d` | a second surface: a state card, a choice form, a chip, a switch's track, a flat stat, a read-only input, a row icon's disc, a hover on a button; lighter than the page and darker than a card in light, so it shows on both |
| `--glass` | `rgb(228 233 240 / .46)` | `rgb(20 24 32 / .46)` | tiles (with `backdrop-filter: blur(2px)`) |
| `--glass-strong` | `rgb(248 250 252 / .8)` | `rgb(32 37 49 / .8)` | panels, bars, menus, sheets (blur 12 px) |
| `--line` | `rgb(20 30 50 / .1)` | `rgb(255 255 255 / .08)` | borders on glass |
| `--line-strong` | `rgb(20 30 50 / .22)` | `rgb(255 255 255 / .18)` | input and button borders |
| `--hover` | `rgb(20 30 50 / .05)` | `rgb(255 255 255 / .06)` | a row or menu item under the pointer |
| `--scrim` | `rgb(16 24 40 / .35)` | `rgb(0 0 0 / .55)` | behind a sheet or dialog |

### Ink

| token | light | dark | for |
| --- | --- | --- | --- |
| `--text` | `#141821` | `#e8ebf1` | body |
| `--text-soft` | `#353c4a` | `#c3c9d4` | descriptions on glass (home's darker muted) |
| `--muted` | `#5a6473` | `#9aa3b2` | labels, hints, table heads, inactive tabs |

### Action

| token | light | dark | for |
| --- | --- | --- | --- |
| `--accent` | `#2b59c3` | `#7ea6ff` | primary button, current tab line, focus, switch on |
| `--accent-text` | `#2450b0` | `#9dbbff` | links and accent words (AA on both backgrounds) |
| `--accent-contrast` | `#ffffff` | `#0b1020` | text on `--accent` |
| `--accent-soft` | `color-mix(accent, --tint-soft)` | same | a selected row, a current sidebar item |
| `--ring` | `0 0 0 3px color-mix(accent, --tint-edge)` | same | focus on an input, whose border turns accent too (the ring alone is under 3:1, so a control whose only sign of focus is a ring draws a solid 2 px accent one) |

### Status

Two values each: the vivid one for dots and fills (3:1 against a surface is
enough for a shape), the `-text` one for words (4.5:1 on `--bg`, `--surface`,
`--surface-2` and strong glass, and on its own badge wash; `tests/contrast.rs`
checks every text token on every one of those grounds).

| token | light | dark |
| --- | --- | --- |
| `--ok` / `--ok-text` | `#16a34a` / `#137236` | `#22c55e` / `#4ade80` |
| `--warn` / `--warn-text` | `#ea580c` / `#9e4908` | `#fb923c` / `#fdba74` |
| `--err` / `--err-text` | `#dc2626` / `#b91c1c` | `#f87171` / `#fca5a5` |
| `--info` / `--info-text` | `#1c549b` / the same | `#86b6ef` / the same |
| `--ask` / `--ask-text` | `#6b4fb3` / `#573ba1` | `#b3a2ea` / `#c4b6f0` |

`--ask` is a question waiting for a person: the bar of a choice form and of a
state card that needs a decision, and the words of its badge, its answer and
an approval's kind word.
It is violet so it reads apart from the accent (an action) and from warn (a
problem); `--ask` reaches 3:1 on every surface, as a bar must.

Washes (a badge's background) are mixed from the vivid value at a named
strength: `color-mix(in srgb, var(--ok) var(--tint-soft), transparent)`,
never a literal colour or percentage. There are three tints and one edge:

| token | value | for |
| --- | --- | --- |
| `--tint-faint` | 6 % | a quiet fill: today's column, an automation's slot, a plain (muted) badge |
| `--tint-soft` | 15 % | a wash under words: a badge, `--accent-soft`, a slot, a failed step |
| `--tint-strong` | 22 % | a halo: a dot's ring, a current step's |
| `--tint-edge` | 45 % | a rim: a tile's or stat's hover border, `--ring`, a past slot's bar |

A plain badge takes faint, not soft: `--muted` words on a soft muted wash
fall under 4.5:1 in light. The few other mixes are not tints of a colour
over its ground: a lit lamp's glow (60 % and 35 % of the lamp's own colour),
a danger card's rim (`--err` 50 % into `--line`) and the quiet backdrop
(`--bg` 55 %).

`--now` (`var(--err)`) draws the time grid's now line.

### Hue

A thing drawn in its own colour (a calendar's source, a tag, a room) sets one
number, `--hue`, inline on its element (`style="--hue: 150"`); four tokens
follow it there, in both themes, and fall back to hue 220 without it. They are
declared on `:root` and re-derived on every element whose `style` names
`--hue`, so a class that sets `--hue` does not move them: set it inline.

| token | light | dark | for |
| --- | --- | --- | --- |
| `--hue-ink` | `hsl(h 60% 34%)` | `hsl(h 70% 68%)` | lines, dots, a slot's bar, a swatch (3:1) |
| `--hue-text` | `hsl(h 55% 26%)` | `hsl(h 55% 80%)` | words in the hue (4.5:1) |
| `--hue-wash` | `color-mix(ink, --tint-soft, --surface)` | same | a slot's fill |
| `--hue-soft` | `color-mix(ink, --tint-faint, --surface)` | same | a quieter fill (an automation) |

The contrast holds at every hue from 0 to 359 in both themes:
`--hue-text` reaches 4.5:1 on every surface, on `--hue-wash` and on a
`.badge.hue`; `--hue-ink` reaches 3:1 on every surface and on `--hue-wash`
(a slot's bar sits on it); `--text` and `--text-soft` reach 4.5:1 on both
washes. `tests/contrast.rs` computes all of
it from the sheet, so a value that breaks it fails CI. HSL lightness is not
even across hues (yellow and green are the bright ones), so the light values
are set by the worst hue.

A tile icon and the login mark keep their own lighter tint (`--mark-bg`,
`--mark-ink`) from the same `--hue`.

### The picture

`--art-tint`: `rgb(238 241 246 / .42)` light, `rgb(12 15 21 / .55)` dark. A
wash laid over the castle; it replaces home's two non-colour tokens
(`--art-opacity`, `--art-filter`) so every theme token is a colour and
`light-dark()` covers all of them, with no `prefers-color-scheme` block.

### Depth, shape, space, type

- Shadows: `--shadow` (1 px + 16 px, soft) and `--shadow-lg` (4 px + 32 px, for
  a lifted tile, a sheet, a popover). Each layer is its own token because
  `light-dark()` holds one value, not a list.
- Radii: `--r-s 6px` (inputs, buttons, chips), `--r-m 10px` (menus, stats,
  tile icons, solid cards), `--r-l 16px` (tiles, panels, lamp tiles, sheets,
  login), `--r-pill 999px` (badges, the theme switch, bar buttons).
- Space: `--s1 .25rem` to `--s6 2rem` on a 4 px grid (lights and maia use it
  already).
- Type: `--font` is the system stack; `--mono` for ids and addresses. Sizes
  `--t-xs .8125rem`, `--t-s .875rem`, `--t-m 1rem` (body), `--t-l 1.125rem`,
  `--t-xl 1.375rem`, `--t-2xl 1.75rem`, `--t-hero clamp(1.75rem, 4vw, 2.5rem)`
  (home's title and a page's h1). Line height 1.5; headings 1.25. Weights:
  body 400, labels 600, titles 700 (a 650 snaps to 700 on Windows and
  Linux anyway, so it is 700 everywhere). Numbers in tables and stats use
  `font-variant-numeric: tabular-nums`.
- Tracking: every small uppercase label (a section head, a sidebar head, a
  table head, a schedule's day, the time grid's head, a lamp's state, an
  approval's kind word) is spaced `--track-caps` (.05em).
- Controls: `--tap 2.5rem` with a mouse, `2.75rem` (44 px) on a coarse
  pointer, in every density. `--nav-h 3.25rem` top bar, `--tabbar 3.5rem`
  bottom bar plus the safe-area inset. `--page 72rem` content width,
  `--sheet-w 27.5rem` a sheet.
- Stacking: `--z-page` 1, `--z-docked` 9 (a docked sheet, under the bars),
  `--z-bar` 10, `--z-menu` 20 (menus, popovers), `--z-scrim` 25, `--z-sheet`
  30, `--z-toast` 40, `--z-dialog-scrim` 50 (`.scrim.above`), `--z-dialog`
  60. A confirm opened from a sheet puts its scrim on `.scrim.above`, so it
  covers the sheet.

### App tokens

Declared beside the shared ones, by the app that needs them, never shared:

- lights: `--on #f5a623` (a lamp's power switch when on), `--kelvin #f0b35a`
  (the colour-temperature slider), `--ink-dark`/`--ink-light` (text on a lamp
  tile filled with the lamp's own colour).
- maia: `--brand` crimson (`#c8102e` / `#e0263c`), `--brand-text`,
  `--brand-soft`: her mark and the current sidebar item, never an action;
  `--avatar-bg` (the avatar's gradient; it falls back to `--brand`). The
  feeling scale (`--feel-*`, read by the avatar's ring as `--feel`) and
  `--violet`. maia also keeps the names its
  plugin-panel contract promises (`--border`, `--muted-2`, `--field-border`,
  `--border-strong`, `--border-hover`, `--hover`, `--surface-raised`): they
  become aliases of the shared tokens (`--border: var(--line)` and so on), so
  a panel that reads them keeps working and follows the new values.

Set on an element by the markup, not on `:root`: `--hue` (a number, the hue
of a tile icon, a login mark or anything drawn in its own hue; 220 if unset),
`--fill` (a lit lamp tile's reported colour) and, on the time grid, `--start`,
`--len`, `--lane`, `--lanes` on a slot and `--at` on the now line (section 6).

## 3. Themes

- Three states: **system** (follow the OS), **light**, **dark**. System is the
  default and means no attribute; a choice is `<html data-theme="light|dark">`,
  which sets `color-scheme` and so flips every `light-dark()` token. The
  stylesheet has no `prefers-color-scheme` block.
- The crate's `Theme` type is the one way an app reads and writes the choice:
  `Theme::from_cookie_headers` on a request, `Theme::set_cookie` for the
  response, `Theme::data_theme` and `Theme::color_scheme` for the page.
- **One switch, one look** (home's): a pill holding three round buttons with a
  monitor, a sun and a moon (the crate's `icons::MONITOR`, `icons::SUN` and
  `icons::MOON`); the chosen one lifted on strong glass in accent. The
  switch's markup and how it posts the choice stay the app's own.
  On a phone it moves into the app's menu (lights' "More", llm-proxy's user
  menu, maia's drawer foot), never into the bottom bar.
- **One cookie for the whole castle.** The choice is kept in a cookie named
  `theme` with value `light` or `dark` (absent means system), `Domain=my-castle.net`,
  `Path=/`, `SameSite=Lax`, `Secure`, a year long, not HttpOnly (it carries
  no secret and llm-proxy's page reads it in the browser). Every app honours
  it: a server-rendered page (home, lights, maia) puts `data-theme` on `<html>`
  at render time, so there is no flash; llm-proxy applies it from a script in
  `<head>` before paint. Whichever app's switch was used, the others follow.
  Today home sets a host-only cookie and maia and llm-proxy each keep a
  `localStorage` key; those become reads of the shared cookie (a stored local
  choice is migrated once, then dropped).
- **Never a second `theme`.** A host-only fallback (an app off the castle
  domain, or one that cannot set the shared cookie) is named `theme_local`,
  never `theme`. Readers prefer `theme` and use `theme_local` only when
  `theme` is absent. A legacy host-only `theme` (no `Domain`) is migrated:
  its value is written to the shared cookie if none exists and the host-only
  one is expired (`Max-Age=0` without `Domain`). The crate implements all of
  it: `THEME_LOCAL_COOKIE`, `Theme::set_local_cookie`,
  `Theme::from_cookie_headers` and `Theme::legacy_cookie_migration`.
- The page's `<meta name="theme-color">` and the installed app's manifest keep
  following the chosen theme as lights does today.

## 4. Density

`<html data-density="dense">` switches the scale for admin screens:

| | comfortable (home, lights) | dense (maia, llm-proxy) |
| --- | --- | --- |
| body | 1rem / 1.5 | .875rem / 1.5 |
| `--t-xs`…`--t-2xl` | .8125 / .875 / 1 / 1.125 / 1.375 / 1.75 rem | .75 / .8125 / .875 / 1 / 1.25 / 1.5 rem |
| h1 | hero clamp | 1.5rem |
| `--tap` (mouse) | 2.5rem | 2.25rem |
| `--tap` (touch) | 2.75rem | 2.75rem |
| `--page` | 72rem | 80rem |
| table cell padding | `--s2 --s3` (.5rem .75rem) | the same; dense comes from the smaller type |
| the castle | full | faint (`.backdrop.quiet`) |

Nothing else changes: same tokens, same components, same radii. A dense app on
a phone still gets 44 px targets.

## 5. Layout and responsive rules

- **Breakpoints** (shared): phone `< 48rem` (bottom bar, stacked tables, one
  column), narrow `< 56rem` (a sidebar becomes a drawer), ultra-wide
  `>= 100rem` (the page grows to `max(--page, 80vw)`, plus the sidebar in a
  sidebar app, and tile grids use a 20rem minimum: 4 columns at 1920 px, 6
  at 2560 px).
- **Override after base**: a breakpoint's override of a selector comes after
  that selector's base rule, never before, since at equal specificity the
  later rule wins (`tests/assets.rs` checks the sheet and the mockup's copy).
- **Page**: `main` centred at `--page`, padding `--s5 --s4`, plus the bottom
  bar's height on a phone. Home's front page pads the top with
  `clamp(2rem, 8vh, 5rem)`.
- **Grids**: `.card-grid` is `repeat(auto-fill, minmax(min(100%, 17rem), 1fr))`
  with a 1rem gap; `.wide` for cards that hold rows (26rem); `.grid-2` for a
  pair of panels (20rem each). Rooms, home's cards and lamp rooms all use it.
- **Backdrop**: fixed, `65% bottom / cover`, masked from transparent at the
  top to opaque at 45 %, washed with `--art-tint`. `.quiet` halves its
  presence for dense screens; a login page always shows it in full.
- **Bars**: the top bar is sticky strong glass, 3.25rem, with the home link
  first, the brand, the app's tabs, then the right-hand side (theme switch,
  user menu). On a phone the tabs become a bottom bar of strong glass (3.5rem
  plus the safe area) with icon over label and the current one marked by an
  accent line on top; the top bar keeps brand, home link (icon only) and side.
- **Sidebar** (maia): 14.5rem of strong glass on the left; under 56rem a
  drawer behind a menu button in a 3.25rem app bar.
- **Motion**: 0.2 s on hover lifts and 0.15 s on switches, and a wait's dot
  pulsing (`castle-pulse`, 1.6 s); everything off under
  `prefers-reduced-motion`.
- **Focus**: 2 px accent outline, offset 2 px, on every focusable thing,
  and on the label of a hidden input (a pill check, a swatch, a choice);
  inside a segmented control, which clips an outline, a 2 px accent ring
  inside the choice's edge. Inputs use `--ring` with an accent border.
- **Base**: headings `h1`–`h6`, `p`, lists, `dl` and `blockquote` have no
  browser margins, only `--s3` under them; lists indent 1.25em; `hr` is a
  `--line` hairline; `blockquote` a `--line-strong` bar in `--text-soft`;
  `summary` takes the pointer. A link is `--accent-text`, visited or not,
  its underline thicker under the pointer.
- **Forced colours** (Windows high contrast): the browser paints surfaces and
  borders itself, so the glass and its hairlines hold up; the sheet puts back
  in system colours what was drawn only in a fill or a shadow (a chosen
  choice, a pressed button, a switch that is on, focus rings), lets dots,
  swatches, slots, the now line, a lit lamp and a progress bar keep their
  colours, and hides the castle.
- **Print**: the light theme on white, no castle, glass, shadow, bars,
  sheets, toasts or spinners, the page at the paper's width and the time
  grid and tables unclipped; a hue, a status or a bar prints in colour.

## 6. Components

Each entry: what it is, how it is drawn, which app has it today.

- **Home link** (`.home-link`): house icon (the crate's `icons::HOME`) plus
  "Home", first in every bar,
  separated from the app's own tabs by a hairline; icon only on a phone; a
  full page load, never boosted. Every app has it but home (it is home).
  On a login page it reads "Back to the castle" under the form.
- **Brand**: the app's name as it appears on home's card ("lights", "maia",
  "llm-proxy"), 700, after the home link. (Lights shows "z2m-mcp" today;
  showing the card's name ties the two pages together. Behaviour unchanged.)
- **Tabs** (`nav.tabs`): muted text, accent underline for the current one;
  a `.count` pill: red (`--err`) for things that went wrong, `.count.ask`
  (violet) for things waiting on a person. Phone: bottom bar as above.
  Lights and llm-proxy.
- **Sidebar** (`.sidenav`): sections with uppercase 13 px headings (12 px
  dense), items with an icon and label, the current one on `--brand-soft`
  with a 3 px brand bar; a `.nav-badge` count, `.nav-badge.ask` for things
  waiting. maia only.
- **Theme switch** (`.theme-switch`): section 3.
- **Bar buttons**: `.icon-btn` (round, glass, for the settings gear and the
  menu), `.pill-btn` (text in a glass pill, for home's Edit and llm-proxy's
  user menu). Pressed or open: strong glass, accent text, shadow. A page's
  one primary action ("New event") is a `.btn.primary.sm` in the bar's side
  on a desk and an `.icon-btn` "+" there on a phone; there is no floating
  button, which would cover the last row of every list.
- **Menu** (`.menu`): a strong-glass dropdown with 10 px radius; items
  `--tap` high; a hairline before "Log out". It is absolutely positioned
  under its control's wrapper, which must be positioned (`position:
  relative`). Lights' More menu and llm-proxy's user menu.
- **Tile** (`.tile`): the glass card you tap. Head row (icon disc, name, URL,
  dot), description, metrics strip under a hairline. Hover: strong glass,
  accent border at `--tint-edge`, lifted 2 px. Home's cards; a room on lights' rooms
  page can be one.
- **Card** (`.card`): strong-glass panel for content; `.solid` for tables.
  `.danger` rims it with `--err` for a removal section. `.flush` drops the
  padding and clips its corners, for a grid or a table that runs edge to
  edge (the calendar's week and month).
- **Popover** (`.popover`): strong glass, 16 px radius, drops from a bar
  control; fixed and full width on a phone. Home's settings panel.
- **Sheet** (`.sheet` with `.sheet-head` and `.sheet-body`): a right-hand
  panel for a record's form, strong glass, full width on a phone, a scrim
  behind; `Escape`, the close button and the scrim close it. The close
  button is `.close` in the head (an `.icon-btn` or a `.btn.ghost.round`).
  A page fetched whole into the sheet goes in a `.sheet-body.flat`, which
  flattens the page's own `.card`. **Docked** (`.sheet.docked`): beside a
  page that stays live (the calendar's editor next to its week, which
  refreshes behind it): no scrim, `aria-modal="false"`, under the top bar and
  above the bottom bar, so the bar's controls stay in reach; on a desk the
  page pads its right side by the sheet's width while one is shown (not
  `hidden`), so the sheet sits beside the page's last column. llm-proxy's
  panels; the calendar's editor, docked; maia's detail rows could use it.
- **Dialog** (`.card.dialog`): a confirm, 26rem, centred near the top (20 vh)
  on a scrim: a title, one sentence, the buttons. A native `<dialog
  class="card dialog">` takes `--scrim` on its `::backdrop`; a scripted one is
  a fixed `.card.dialog` over a `.scrim`, a `.scrim.above` when it opens
  from a sheet. llm-proxy's and the calendar's
  confirms. A confirm about an address (a link that leaves the hosts an app
  trusts) shows it whole in a `.address` line, mono on `--bg-sunken`, so
  the person reads where it goes before Open.
- **Status dot** (`.dot`): .7rem with a 4 px wash ring; `.sm` .5rem for a
  summary line. States ok / warn / err / info / ask / unknown (muted). Home's card dots and
  header dot; lights' "not seen yet"; maia's chip dots; llm-proxy's pills.
- **Badge** (`.badge`): 13 px (12 px dense) 600 in a pill on a `--tint-soft` wash (a plain one `--tint-faint`);
  ok / warn / err / info / ask / accent / muted; `.outline` for a count; `.hue` in the element's
  own hue (`--hue-text` on a wash of `--hue-ink`). One component for lights'
  badges, maia's chips and llm-proxy's pills. maia's `.chip` keeps working as
  an alias (its plugin contract names it).
- **Note** (`.note`): a strong-glass line with a 4 px coloured left bar: what
  happened and why, an optional action; ok / warn / err / info / ask, muted
  without one. Transient outcomes (lights' flash,
  `.sticky` to stay in view while the page scrolls) and standing warnings
  (maia's attention rows, llm-proxy's errors) alike.
- **Wait** (`.note.wait`): a note for a wait with a known end (a poll's next
  check, a backoff): an accent bar and a `.dot` that pulses (still under
  reduced motion), the `.what`, and in `.why` the word before a mono
  `.countdown` the page counts from an absolute time ("next check in 3:52").
  Once the time has passed it is `.due`: bar and dot muted, the dot still,
  the countdown saying how long ago ("was due 09:30 · 2 min ago"), never a
  negative count. No app has it yet; an agent's chat (OpenUI) is the first candidate.
- **Toast** (`.toast`): a note that floats bottom-centre over the page for a
  few seconds, above the bottom bar on a phone; its bar in a tone (ok / warn
  / err / info / ask), accent without one. maia's and llm-proxy's toasts
  (llm-proxy's moves from bottom-right).
- **State card** (`article.state-card`): one thing's state (a job, a task, a
  run) in a few lines, on `--surface-2` with a 4 px bar in the state's
  colour. A `header` holds a `.dot.sm`, the title as a `b`, a `.badge` with
  the state's word and an optional `.task` id (mono, muted); a `ul` under it
  holds the lines (`--text-soft`, `--t-s`). The state is a class on the card,
  which colours the bar, the dot and the badge: `ok` and `done` in `--ok`,
  `working` and `waiting` in `--accent`, `needs-decision` in `--ask`,
  `blocked` and `failed` in `--err`; any other state is muted with a plain
  badge. The badge always says the word, so the state is never colour
  alone. No app has it yet; an agent's chat (OpenUI) is the first candidate, for an agent's status block; a dashboard's job list could
  use it.
- **Buttons** (`.btn`): neutral (surface, strong line), `.primary` (accent),
  `.danger` (err), `.ghost` (transparent), `.link`, `.sm`, `.round`.
  `--tap` high, 6 px radius, 600. Neutral unless it is the one thing the form
  is for; one primary per form.
- **Switch** (`.switch`): a pill 3.25 × 2rem (2.5 × 1.5rem dense) with a
  knob; on = `--accent`; `.unknown` dashed with the knob in the middle.
  Lights' lamp power switch uses `--on` amber when on (a lamp is on, not a
  setting is enabled); the switch sits in a `--tap` square so the target is
  44 px. llm-proxy's green `input.switch` becomes this.
- **Segmented** (`.segmented`): a few choices as one control, the chosen one
  on `--accent-soft`. The choices are buttons (`aria-pressed` or `.on`), links
  for a view switch (`aria-current`), or radio labels (`<label><input
  type="radio">…</label>`, a form field with no script; the radio is hidden,
  the label is the target and takes the focus ring). `.sm` is a shorter one
  for a toolbar on a desk; under a coarse pointer it is `--tap` high like
  the rest. A control that may run out of room is `.wrap` (it breaks
  onto a second row and keeps every hairline) or `.fill` (equal columns),
  `.fill-phone` for equal columns on a phone only. Lights' On/Off/Toggle,
  maia's theme radios, llm-proxy's filter row, the calendar's ‹ Today › and
  its repeat choices.
- **Inputs** (`.input`, `.field`): opaque surface, strong-line border, 6 px
  radius, `--tap` high; label above in 13 px (12 px dense) 600 muted, hint
  below; focus ring. A `select.input` keeps the browser's arrow. States:
  `:disabled` sunken, dashed and muted; `[readonly]` on `--surface-2`;
  `aria-invalid="true"` rims it in `--err` (an err ring on focus), with a
  `.error` line under it saying what is wrong in words; a required field
  marks its label with a `.req` (an `aria-hidden` "*", the input carries
  `required`). The placeholder is `--muted` at full strength (4.5:1). Under
  a coarse pointer an input's text is at least 16 px, so iOS does not zoom
  the page on focus. `.fields` wraps them in a row; `.actions` holds the
  buttons.
  `fieldset.group` holds a set of them under a `legend` drawn as a field
  label, with no box.
- **Pill checks** (`.pills` of `.check.pill`): a few booleans as pills, the
  checkbox hidden, a checked one on `--accent-soft` with an accent rim.
  Weekdays on a repeating event or a lights schedule; sources to include in a
  feed.
- **Swatch picker** (`.swatches`): one radio per hue, each a 44 px round
  target whose label sets `--hue` and holds a `.swatch.hue.lg`; the chosen one
  is ringed in its own `--hue-ink`. A calendar source's colour; a room's or an
  app tile's hue if a settings form wants one.
- **Swatch** (`.swatch`): a 14 px rounded square with a strong line, for a
  colour the device reports (lights). `.swatch.hue` is a .8rem dot in the
  element's `--hue-ink` (`.sm` .6rem, `.lg` 1.5rem).
- **Slider** (`.slider`): label and value on one line, the range below; the
  colour-temperature one takes `--kelvin`. Lights.
- **Toolbar** (`.toolbar`): filter input, a meta count, a segmented filter;
  wraps. maia's karma tools, llm-proxy's bars.
- **Chips** (`.chips`): small pills in a row. Plain, a `li` is a static pill
  (an alias with a remove button on lights, a route on llm-proxy).
  **Toggle chips** (`.chips.toggle`): filters you switch on and off; the
  chip's link or button carries `aria-pressed` and may hold a `.swatch.hue`
  and a `.count`, and an optional second target (`.act`: "only", "×") sits
  beside it in the chip, always visible since a touch screen has no hover. Off
  (`aria-pressed="false"` or `li.off`) is dashed and muted with the swatch
  hollow; `li.plain` holds a trailing "show all" link. `.scroll` keeps them on
  one row you swipe on a phone instead of several stacked rows. The
  calendar's sources; image-desk's and vision-desk's tags.
- **Link chips** (`.chips.links`): places to go, each pill holding one `<a>`
  that fills it, with a mono `.glyph` for its kind (a change, a check run, a
  commit, a page) before a short label. A link the app will not open (not
  `https`) is `li.refused`: dashed and muted, the label and its address's
  `.host` in mono, no anchor. No app has it yet; an agent's chat (OpenUI) is the first candidate, for an agent's links.
- **Choice** (`fieldset.choice`): a question put to a person, answered in
  one tap or with several picks and a submit. The prompt is the `legend`;
  `.options` holds the options as `.btn.option`s and, for several picks, a
  `.btn.primary` submit at the end of the same row; a `p.about` per option
  with a description (`<b>label</b>: what it means`), and `p.answered`
  (mono, `--ask-text`) once it is answered or on its way. The options are
  buttons carrying `aria-pressed` (a script answers on a tap, or toggles the
  picks) or, with no script, `label.btn.option`s holding a hidden radio (one
  answer) or checkbox (several) that a form posts; a chosen one is drawn
  pressed (an accent rim on `--accent-soft`). Long labels wrap. Answered, the
  fieldset is `disabled`: the other options fade, the chosen ones stay at
  full strength. A 4 px `--ask` bar on `--surface-2` marks it as a question.
  No app has it yet; an agent's chat (OpenUI) is the first candidate, for an agent's question.
- **Approval** (`fieldset.choice.approval`): a choice that asks leave for one
  step that cannot be undone or that leaves the house (a merge, a send, a
  delete). The legend opens with a mono `.kind` word (`APPROVAL`, in
  `--ask-text`) before the question; `ul.what` says in a few lines what will
  happen and what cannot be taken back; `details.preview` folds the command
  or the diff in a `pre` (18rem at most, then it scrolls), closed until
  opened. `.options` holds three fixed answers: **Approve** (`.btn.primary`),
  **Deny…** (`.btn.ghost.deny`, its words and rim in err) and, when one
  value can be changed, **Edit…** (`.btn.ghost`). Deny and Edit each open a
  `.reply` row under the options: an `.input` (a reason, optional; or the
  value to change, prefilled), its button and a `.hint`; `.reply` is the
  same row for any choice answered in words. Answered, the fieldset is
  `disabled`: the chosen button carries `aria-pressed="true"` and stays at
  full strength, the others fade, and `p.answered` says what was sent. An
  approval decides nothing by itself: the answer is words to whoever asked.
  No app has it yet; an agent's chat (OpenUI) is the first candidate, for an agent's ask.
- **Rows** (`ul.rows`): icon disc, title and facts, state, action; one line on
  a desk, two on a phone. Lights' devices and rooms; llm-proxy's recent list.
- **Table** (`table.data` in a `.card.solid`, inside `.table-wrap`): heads
  uppercase 13 px (12 px dense) muted, numbers right and tabular, hover wash, selected row
  on `--accent-soft`. `.stack` turns each row into a card on a phone
  (`td.lead` title, `td.trail` beside it, `data-label` names a fact, `.end`
  the buttons); maia's `.stack` and llm-proxy's `data-label` tables both map
  onto it.
- **Grid table** (`.grid-table > table`): a small table of plain cells whose
  columns nobody declared (an agent's comparison, a pasted grid), where
  `table.data` needs to know which column is the title and which a number.
  Every cell ruled in `--line-strong`, the head row 600 on a faint `--hover`
  wash, numbers tabular in every cell (`.num` right-aligns a column), an
  optional `caption` above on the left in muted `--t-xs`. The table is as
  wide as its cells: heads stay on one line and a cell wraps only past
  20rem, and the `.grid-table` box scrolls sideways when that is wider than
  its place, as on a phone, instead of squeezing every column to a word.
  No app has it yet; an agent's chat (OpenUI) is the first candidate, for an agent's table.
- **Pager** (`.pager`): "5 of 14" left, page numbers right as `--tap`
  squares, the current one on `--accent-soft`. New; no app pages today, maia's
  long tables are the first candidate.
- **Stats** (`.stats > .stat`): strong-glass cells, label 13 px (12 px dense) muted above a
  1.5rem value and a sub line. maia's overview and llm-proxy's overview; the
  compact `.metric` inside a tile is home's. A tone (`.stat.ok`, `.warn`,
  `.err`) colours the value in its `-text` token, and the sub line says why,
  so the colour is never alone. `.stat.flat` is a cell on `--surface-2`
  without glass, for a panel that is glass already; `.stats.inchat` fits a
  strip into a chat bubble (tighter cells, two a row on a phone) with a
  `.caption` spanning the row under them ("as of 09:41"). The tones,
  `.flat` and `.inchat` have no app yet; an agent's chat (OpenUI) is the first
  candidate, for an agent's report.
- **Facts** (`dl.facts`): term and value in two columns, one on a phone.
  Lights' About, maia's run details.
- **Schedule** (`ul.schedule`): what happens when. Each `li` is a time
  column (`.t`, tabular, a `b` for a second line) and a slot; `li.day` heads
  a day (a bold number, the weekday, a muted word such as "today"), `li.more`
  closes one ("3 more tomorrow"). A past row's slot is quieter (`--hue-soft`,
  a faded bar, a lighter title), never faded below AA; the running row, or
  with none the next one, sits on `--accent-soft` with its time in accent.
  The calendar's agenda, home's today list, lights' schedule runs.
- **Slot** (`.slot`): one event or run as a block in its hue: a 3 px
  `--hue-ink` bar on the left, a `--hue-wash` fill, the title in 600 and a
  `--text-soft` line under it. `.row` puts it on one line (a month cell, the all-day
  row, a schedule row); `.auto` marks an automation, something no person
  put there (a dashed bar, the quieter `--hue-soft`, a bolt drawn in
  `currentColor`); `.more` is the "+2 more" link. A link slot lifts on hover.
- **Time grid** (`.timegrid`): columns of time (a week's days, one day, or
  any set of tracks) against an hours axis. A `.tg-head` row names the
  columns (a muted weekday over a bold number; `.today` in accent), a
  `.tg-allday` row holds `.slot.row`s for whole-day items, and the
  `.tg-body` holds `.tg-hours` (the axis) and one `.tg-col` per column with
  an hour line every `--hour-h` (2.25rem). Each row's first cell is the
  gutter. The grid sets `--cols` (7), `--from` (its first hour) and `--hours`
  (how many it shows); a `.slot` in a column sets `--start` and `--len` in
  hours (9.5 is 09:30) and, beside others at the same time, `--lane` of
  `--lanes`; `.tg-col.today` is washed in accent; `.tg-now` sets `--at` and
  draws the now line in `--now` across that column. It goes in a
  `.card.flush`; on a phone, once the columns fall under 5.5rem each, the
  grid scrolls sideways. Under a coarse pointer a slot is at least `--tap`
  tall however short its time, so an app placing lanes counts a slot as
  lasting at least `--tap` ÷ `--hour-h` hours (about 1.2 h at the defaults)
  and puts a neighbour that starts inside that span in the next lane. The
  calendar's week and day; lights' runs this week could use it.
- **Empty state** (`.empty`): dashed strong line, icon, bold first line, what
  to do next. All apps.
- **Code and logs** (`pre`, `.log`): `--bg-sunken`, mono 13 px, soft wrap;
  `.t` timestamps muted, `.k` keys accent, `.e` errors err-text.
- **Diff** (`pre.diff`): a unified diff, one `span` per line: `.add` in
  `--ok-text`, `.del` in `--err-text`, `.hunk` (`@@`) in `--accent-text`,
  `.meta` (`---`, `+++`, `diff --git`) muted; the `+`/`-` marks stay in the
  text, so the colours are never alone. `.diffhead` names the change in one
  mono line: the `.path`, the `.plus` and `.minus` counts, the hunks, a link.
  `details.diffblock` folds the `pre` under that line as its summary (▸/▾),
  18rem at most, then it scrolls. An approval's preview of a diff uses the
  same `pre.diff`. No app has it yet; an agent's chat (OpenUI) is the first candidate, for an agent's change.
- **Checklist** (`.checklist`): numbered discs, done in `--ok` (✓), the
  current one accent-rimmed (`li.cur` puts its label in `--accent-text`), a
  failed one an `!` in `--err-text` on an err wash with an err rim
  (`.n.failed`), a skipped one dashed with its label struck through
  (`.n.skipped`, `li.skipped`); a step still to do keeps its number. A
  `.step-note` under a label says why in one muted line. For a plan someone
  follows over time, `li.head` leads the list with its title and a `.count`
  ("2 of 4 done · 1 failed", always in words, so the discs' colours are
  never alone), and `.compact` tightens the rows for a chat bubble. When a
  newer copy of the same list exists, the old one folds into
  `details.superseded`: one dashed mono line saying so, with a `.btn.link`
  to the latest, and the old list faded inside. llm-proxy's Get started has
  the plain list; its states, `li.head`, `.compact` and `details.superseded`
  have no app yet, and an agent's chat (OpenUI) is the first candidate, for
  an agent's plan.
- **Stepper** (`ol.stepper`): a thing's steps on one line, a disc per step
  with its name under it, joined by a hairline. A step is done (filled in
  `--text-soft`), here (larger, in its tone — accent by default, `.ok`,
  `.ask`, `.warn` or `.err`, its name bold) or still ahead (hollow), and the
  stretch after a done step — the covered ground up to the current one — is
  darker. The colour is never alone: the here disc is larger and its name
  bold, and the app says which step is current without it, with
  `aria-current="step"` on the here step and the state in the list's
  accessible name, as the mockup draws it. A stepper sits on a card, like
  the checklist, where every tone disc keeps its 3:1. No app has it yet; an agent's chat (OpenUI) is the first candidate, for an
  agent's change or task.
- **Progress** (`progress.progress`): a native `<progress value max>`, an
  accent bar on a `--line-strong` track, a pill .5rem high, beside the words
  that say the same ("2 140 of 6 300"), never instead of them; with no
  value it is indeterminate, an accent stretch sliding along the track
  (still under reduced motion). A state card's or a job's progress. New; no
  app has it yet, a polling page's long job is the first candidate.
- **Loading** (`.spinner`, `.htmx-indicator`, `aria-busy`): a `.spinner` is
  a ring that turns, 1em in `currentColor`, beside words or with an
  `aria-label`, never alone (slower under reduced motion, never still, since
  a still one reads as stuck). `.htmx-indicator` hides it, or anything,
  until htmx marks the request (`.htmx-request` on it or its parent). A
  region that refreshes in place carries `aria-busy="true"` meanwhile: its
  content stays at full strength under a progress cursor, so a poll never
  flashes the page. New; the htmx apps' polls are the first candidates.
- **Login** (`.login`): one strong-glass card centred on the full castle: the
  app's mark (its home icon on a hue tile), its name, a sub line, the fields,
  one primary button, and the home link under it. Lights puts the passkey
  button first and the password form under "or with the password".

## 7. Touch

For home and lights (and any dense app on a phone):

- Every control is at least 44 × 44 px under a coarse pointer (`--tap`), and
  a small control (a switch, a caret) sits in a 44 px box. The components
  below say how each meets it.
- Six older controls fall short of it today: `.btn.sm` (`--tap` minus .5rem,
  36 px on a touch screen), `.icon-btn` (`--tap` minus .125rem, 42 px), the
  `.theme-switch` buttons (2rem, 32 px), `.btn.link` (no minimum height: as
  tall as its line), the remove button in a plain `.chips` `li` (2rem, 32
  px) and a `.sheet-head .close` outside a sheet (only the sheet's own close
  is bumped under a coarse pointer). They predate this table and get the
  coarse-pointer bump together in a later release; until then a page for
  touch uses `.btn` and `.btn.ghost.round`.
- Nothing is reachable by hover alone: a second action on a chip or a row is
  a visible target.

  | component | its target under a coarse pointer |
  | --- | --- |
  | `.segmented` (links, buttons, radio labels; `.wrap`, `.fill`, `.fill-phone`) | each choice `--tap` high; `.sm` grows from 36 px to `--tap` |
  | `.chips.toggle` | the chip `--tap` high, its link or button stretched to that height; the `.act` target `--tap` wide; `li.plain`'s link the same |
  | `.check.pill` | `--tap` high always (`.check`'s min-height) |
  | `.swatches` | each hue a `--tap` circle always |
  | `.slot` as a link (in a schedule, a month cell, the all-day row, the time grid, `.more`) | at least `--tap` high, however short the event; see the time grid's lane rule |
  | `.tg-head` column links | a muted line over a bold one plus padding, over 44 px high; a column is at least 5.5rem wide |
  | `.sheet-head .close` | at least `--tap` square, whichever button it is |
  | `.card.dialog` buttons | `.btn`, `--tap` high |
  | `.choice` options and submit | `.btn`, `--tap` high, growing with a wrapped label |
  | `.choice .reply` (an approval's Deny and Edit, an answer in words) | `.input` and `.btn`, `--tap` high; `.preview`'s summary at least `--tap` high |
  | `details.superseded` | its link to the latest `--tap` high |
  | `.chips.links` | each link fills its pill, `--tap` high under a finger |
  | `details.diffblock` | its summary line at least `--tap` high, its head centred in it |
  | `.choice .reply` with a `textarea` | the box at least 4.5rem, its button at the box's foot |
  | `.grid-table` | no target; the box scrolls sideways under a finger |
  | `.theme-switch` | each button 2rem (32 px): short of 44 px, in the list above; on a phone the switch lives in the app's menu |
  | `.input` | `--tap` high; its text at least 16 px, so iOS does not zoom on focus |

- **Lamp tile** (`.lamp`): at least 5.75rem tall, 16 px radius, name and
  state in the corners; on = filled with the lamp's reported colour (warm
  white when it reports none) and a soft glow of it, ink picked for contrast;
  off = strong glass; unknown = dashed; disabled = faded with the reason.
  Pressed: scaled to 97 %.
- **Dimmer strip** (`.lamp-dim`): two 44 px halves under the tile, minus and
  plus, the outer corners rounded to the tile; disabled at the ends of the
  range or while the lamp is off.
- Tap highlights off; `touch-action: manipulation` on buttons.
- The bottom bar sits above the safe-area inset; the sticky note sits under
  the top bar on a desk and at the top on a phone.
- A room's lamp grid is `minmax(8.5rem, 1fr)`, two columns on the narrowest
  phones.

## 8. Elements only one app has

How each should look when another app needs it:

| element | app | the rule |
| --- | --- | --- |
| castle backdrop | home | every app, section 5; `.quiet` on dense screens and behind the calendar's grids, where it would cross the hour lines |
| metrics strip on a tile | home | `.metric` inside `.tile`; up to four, label under value |
| settings popover (gear) | home | `.icon-btn` + `.popover`; a form, no script needed |
| edit mode (drag to reorder) | home | `.pill-btn` pressed; dashed accent border on the movable cards, the dragged one at 55 %; a hint line under the head |
| summary dot and count | home | `.dot.sm` plus muted tabular text in the bar |
| lamp tiles, dimmer strip | lights | section 7 |
| room cards with rows | lights | `.card` with `.section-head` and `ul.rows`; `.card-grid.wide` |
| sticky outcome line | lights | `.note.sticky`; use for any page where the answer must stay in view while it scrolls |
| passkey login | lights | the primary button above the password fields on `.login` |
| network map | lights | Cytoscape in a `.card.solid` with a `.toolbar` above and a legend as `.badge.outline`s; node colours from the tokens |
| gradient editor | lights | `.fields` of colour inputs in a row; a 5rem swatch in the legend |
| guest bar | lights | the bar with only the home link, the guest's name muted and Log out |
| sidebar with sections | maia | `.sidenav`; the current item takes the app's brand colour, else accent |
| avatar with a mood ring | maia | `.avatar` with the ring colour from `--feel`; `.sm` in the app bar, `.lg` in the hero |
| attention list | maia | `.note` rows (err or warn) inside a card |
| mood gauge and meters | maia | its own SVG, strokes from the tokens; the feeling scale stays maia's |
| restart banner | maia | `.note.warn` with a primary small button, above the page head |
| plugin panels | maia | unchanged contract; panels read the aliases |
| setup checklist | llm-proxy | `.checklist` |
| user menu in the bar | llm-proxy | `.pill-btn` + `.menu`, with the theme switch inside on a phone |
| row menu (⋯) | llm-proxy | `.btn.ghost.sm` + `.menu`; danger item in `--err-text` |
| secret reveal box | llm-proxy | a `.card.solid` rimmed with `--ok`, the secret in mono, select-all |
| remap confirmation | llm-proxy | a `table.data` with current/proposed columns in a `.card.solid` |

## 9. Where this lives, and how apps pick up a change

**This repository is the single source.** Nothing is copied into the apps:

- `docs/design-language.md`: this document, kept current.
- `assets/castle.css`: the shared stylesheet: tokens, base, the components
  above, nothing app-specific.
- `assets/castle.webp`: the picture.
- `docs/mockup.html`: the living mockup, updated when a component changes.
- `src/lib.rs`: the `castle-design` crate, which compiles the two assets in
  (`castle_design::CSS`, `castle_design::CASTLE_WEBP`) and carries the theme
  cookie helper (`castle_design::Theme`) and the shared icons
  (`castle_design::icons`, also in `assets/icons/`).

**Each app depends on the crate, pinned to a tag** (the dependency line is in
the [README](../README.md#using-it-from-an-app)).

The app serves `CSS` and `CASTLE_WEBP` from its own routes (or inlines the
CSS into its page), sets `--castle` to the picture's URL, and puts its own
sheet after the shared one. The repository is public, so the git fetch needs
no token in a Docker build or CI. Builds stay self-contained once the crate
is fetched, and no app depends on another being up.

**Picking up a change** is deliberate, one app at a time: a new tag here, then
in each app bump the `tag` in `Cargo.toml` and run `cargo update -p
castle-design`, which moves `Cargo.lock`; the app's own tests and a look at
its pages decide when it lands. An app that pins a branch instead gets the
same bump from `cargo update -p castle-design` alone, at whatever commit the
branch is on; tags are the recommendation, because the lock then names a
version the changelog explains.

Values may move; **names are a contract** (as maia's plugin panels already
treat them). Adding a token is free; renaming or removing one is a breaking
change: a new minor version while the crate is 0.x, called out in the
changelog, and every app updated in the same round.

## 10. What each app changes (look only)

- **home**: its CSS moves onto `castle.css` plus a short front-page sheet;
  tokens as above (`--text-soft` replaces its dark muted; `--art-tint`
  replaces opacity/filter); the theme cookie gets `Domain=my-castle.net` and
  loses `HttpOnly` (the crate's `Theme::set_cookie`). Its edit mode, popover
  and summary dot already match the language.
- **lights**: its sheet is rebuilt on `castle.css`: backdrop, strong-glass bars
  and cards, tiles for rooms, the theme switch in the home look, lamp tiles
  with a glow, the login card. Classes and markup stay; its test that the app
  sheet holds no colour literals stays. Brand text "lights".
- **llm-proxy**: its inline `<style>` becomes castle.css plus a short app
  sheet; tables move into solid cards, panels become sheets, pills become
  badges, the switch becomes `.switch`, the toast moves to bottom-centre, the
  login becomes the shared card, theme from the shared cookie. Markup classes
  can stay with aliases where the JS builds them.
- **calendar**: points `--castle` at its picture (on `.backdrop.quiet`); its
  source colours move onto the hue tokens, its filter row onto toggle chips,
  its agenda onto the schedule list and slots, its week and day onto the time
  grid in a `.card.flush`, its forms onto `.segmented` radio labels, pill
  checks, the swatch picker and `fieldset.group`, its confirm onto
  `.card.dialog`, its editor onto a `.sheet.docked`; the phone's floating New
  button becomes an `.icon-btn` "+" in the bar.
- **maia**: its token sheet keeps every contract name as an alias of the
  shared tokens; its app sheet takes the glass sidebar, the faint backdrop,
  shared tables, badges and notes; the login becomes the shared card with her
  avatar as the mark; theme from the shared cookie. Tests that pin CSS
  substrings are updated with the rules they pin.
