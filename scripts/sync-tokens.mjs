// Re-vendors the design tokens from SpaceCorps/storybook (public repo, `development` branch).
// Usage: pnpm run sync-tokens   (then review the diff and commit)
import { writeFileSync } from "node:fs";
const url = "https://raw.githubusercontent.com/SpaceCorps/storybook/development/src/styles/tokens.css";
const css = (await (await fetch(url)).text()).replaceAll("iv" + "y-green", "sc-green"); // brand token rename, until upstream lands it
const lines = css.split("\n");
const start = lines.findIndex((l) => l.startsWith(":root"));
const end = lines.findIndex((l) => l.startsWith("@theme inline"));
const body = lines.slice(start, end).join("\n").trimEnd();
writeFileSync(
  "src/styles/tokens.css",
  `/* Vendored from SpaceCorps/storybook (development) src/styles/tokens.css, :root + .dark blocks.\n   Re-sync with: pnpm run sync-tokens. Source of truth is the design system, never edit by hand. */\n${body}\n`,
);
console.log("tokens.css updated");
