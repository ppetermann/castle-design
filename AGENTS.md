# Project agent memory

castle-design is the shared design system of the castle's web apps (home,
lights, maia's admin UI, llm-proxy's admin UI): `assets/castle.css`,
`assets/castle.webp`, the rules in `docs/design-language.md`, and a tiny
no-dependency crate (`src/lib.rs`) that compiles the assets in and carries
the shared `theme` cookie helper. Apps pin it as a git dependency by tag
(README).

- **The repository is public.** No IPs, private hostnames (only the generic
  `my-castle.net` names), tokens, real people or internal notes in code, docs
  or the mockup's example data.
- **`docs/design-language.md` is the intent**; `assets/castle.css` implements
  it and `tests/assets.rs` enforces its hard rules (no
  `prefers-color-scheme`, colours only inside custom property declarations,
  every `var()` declared or listed in `APP_TOKENS`). A new app-provided token
  goes in that list and in castle.css's header.
- **`docs/mockup.html` carries its own frame-scoped copy** of the sheet
  (`.frame`/`.screen`, `@container` queries, embedded picture). A component
  change lands in `assets/castle.css` and, when it shows, in the mockup's copy
  too.
- **Token and class names are a contract** with four apps: adding is free;
  renaming or removing is a breaking change (new minor while 0.x, noted in
  `CHANGELOG.md`).
- Local ladder = CI (`.github/workflows/ci.yml`): `cargo fmt --all -- --check`,
  `cargo clippy --all-targets -- -D warnings`, `cargo test`.

## Delivery

Fleet flow: `/home/ppetermann/agents/firstmate/.agents/skills/fleet-delivery-flow/SKILL.md`.

- **Ladder:** the three commands above, in order, green on the exact push head.
- **PRs open as drafts, always** (`gh pr create --draft`); drafts run no CI.
  The flip to ready (`gh pr ready <n>`) fires the one CI run (`check`) and the
  one review round.
- **Review gate:** the OCR review bot only (`ocr-review`); no CodeRabbit.
- **Release = merge to main**, then the owner or firstmate tags a version
  (`git tag -a vX.Y.Z <merge-sha> -m "castle-design X.Y.Z" && git push origin
  vX.Y.Z`) matching `version` in `Cargo.toml` and a `CHANGELOG.md` section of that
  number, both set in the PR that releases it. Apps pick it up by
  bumping their `tag` and running `cargo update -p castle-design`.

## Maintaining this file

Keep this file for knowledge useful to almost every future agent session in this project.
Do not repeat what the codebase already shows; point to the authoritative file or command instead.
Prefer rewriting or pruning existing entries over appending new ones.
When updating this file, preserve this bar for all agents and keep entries concise.
