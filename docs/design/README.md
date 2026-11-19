# Design notes (Nov 2026 redesign)

Concept: **flight deck**. The home page opens with a dark viewport into the game (always dark, in both themes), a telemetry bar of real numbers, hairline grids, mono labels and HUD corner ticks on hover. Below it the page follows the visitor's theme, built from the SpaceCorps design-system tokens (ivy green `#00cc92`, zinc neutrals, Geist and Geist Mono, 0.5rem radii).

- `before-*.jpg` the previous site, `after-*.jpg` this branch, at 375, 768 and 1440 px in dark and light (full page, cropped at 9000 px).
- `assets-src/` source of the OG image and the favicon; both were rendered with headless Chrome (see PR description).

## Proposed upstreams to `@spacecorps/components`
1. Mono "eyebrow" label (uppercase, tracked) and the section header pattern.
2. HUD-corner card hover (`.hud`) and the pulsing "Live" status tag.
3. Stat tile with tabular numerals (`.proof`) and the telemetry strip.
4. Searchable directory list with category chips (`.dir`, `.filters`).
5. Tokens: `--space` (#04070a), `--line` / `--line-strong` (foreground at 11 / 22 %), `--glow`, `--accent-text` (ivy-800 in light, ivy in dark; the system already uses this pairing for links and badges).
